//! Asset loading; for simplicity "loading" is done at compilation time,
//! and this makes life of distribution a lot simpler.

use crate::components::*;

use image::io::Reader;
use image::DynamicImage;
use image::FilterType;
use specs::prelude::*;
use std::io::Cursor;

/// Loads an image from bytes and resizes it to an exact size.
fn load_bytes(bytes: &[u8], size: (u32, u32)) -> DynamicImage {
    Reader::new(Cursor::new(bytes.as_ref()))
        .with_guessed_format()
        .expect("Cursor io never fails!")
        .decode()
        .unwrap()
        .resize_exact(size.0, size.1, FilterType::Nearest)
}

/// Data structure for storing and accessing loaded assets, which will also be
/// accessed by the rendering systems to load textures into GPU memory.
#[derive(Default)]
pub struct Loader {
    pub images: Vec<DynamicImage>,
    names: Vec<String>,
}

impl Loader {
    /// Pushes an image with it's associated name into the loader.
    fn push_image(&mut self, name: String, bytes: &[u8]) {
        // TODO: handle images of varying sizes
        let image = load_bytes(bytes, (32, 32));
        self.images.push(image);
        self.names.push(name);
    }

    /// Get the index of a loaded image by name.
    fn find_name_index(&self, name: &String) -> Option<usize> {
        self.names
            .iter()
            .enumerate()
            .find(|(_, n)| &name == n)
            .map(|(e, _)| e)
    }
}

static ASSETS: &[(&str, &[u8])] = &[
    ("a", include_bytes!("../../assets/a.png")),
    ("b", include_bytes!("../../assets/b.png")),
    ("c", include_bytes!("../../assets/c.png")),
    ("d", include_bytes!("../../assets/d.png")),
];

/// System for bundling assets and entities together.
#[derive(Default)]
pub struct AssetSystem {
    reader_id: Option<ReaderId<ComponentEvent>>,
    modified: BitSet,
    inserted: BitSet,
}

impl<'a> System<'a> for AssetSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Name>,
        Read<'a, Loader>,
        WriteStorage<'a, TextureIndex>,
    );

    /// Loads statically embedded images at setup time.
    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(world.write_storage::<Name>().register_reader());
        let mut loader = world.fetch_mut::<Loader>();
        for (name, data) in ASSETS.iter() {
            loader.push_image(name.to_string(), data);
        }
    }

    /// Bundles assets and entities by name.
    fn run(&mut self, (entities, names, loader, mut texture_indicies): Self::SystemData) {
        self.modified.clear();
        self.inserted.clear();

        {
            let events = names.channel().read(&mut self.reader_id.as_mut().unwrap());
            for event in events {
                match event {
                    ComponentEvent::Modified(id) => {
                        self.modified.add(*id);
                    }
                    ComponentEvent::Inserted(id) => {
                        self.inserted.add(*id);
                        println!("{}", id);
                    }
                    _ => {}
                }
            }
        }

        // Every named entity is potentially given a texture index, which is
        // used to access a texture from GLSL.
        for (_, name, _id, t_index) in
            (&entities, &names, &self.inserted, &mut texture_indicies).join()
        {
            println!("{} -- {:?}", name.0, t_index.0);
            if let Some(index) = loader.find_name_index(&name.0) {
                t_index.0 = Some(index as usize);
            }
        }
    }
}
