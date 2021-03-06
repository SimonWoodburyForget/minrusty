use crate::builder::BuilderSystem;
use crate::components::*;
use crate::loader::*;
use crate::map::MappingSystem;
use crate::physics::*;

use instant;
use specs::prelude::*;
use vek::*;

/// Duration between current and last instants.
#[derive(Default)]
pub struct DeltaTime(pub instant::Duration);

/// Instant when the game started.
pub struct GameStart(pub instant::Instant);

impl Default for GameStart {
    fn default() -> Self {
        GameStart(instant::Instant::now())
    }
}

pub struct GameState {
    pub ecs: World,
    dis: Dispatcher<'static, 'static>,

    /// Last update instant.
    last: instant::Instant,
}

impl GameState {
    pub fn new(renderer: crate::render::Renderer) -> Self {
        let mut world = World::new();

        world.insert(GameStart::default());
        world.insert(DeltaTime::default());

        let mut dispatcher = DispatcherBuilder::new()
            .with(crate::logger::Sys::default(), "log-system", &[])
            .with(MappingSystem::default(), "mapping-system", &[])
            .with(AssetSystem::default(), "asset-system", &[])
            .with(
                BuilderSystem::default(),
                "builder-system",
                &["mapping-system"],
            )
            .with(PhysicSystem, "physic-system", &[])
            .with_thread_local(renderer)
            .build();
        dispatcher.setup(&mut world);

        Self {
            ecs: world,
            dis: dispatcher,
            last: instant::Instant::now(),
        }
    }

    pub fn create_block(&mut self, x: i32, y: i32, name: &str, red: f32) {
        self.ecs
            .create_entity()
            .with(Name(name.into()))
            .with(Coordinate(Vec2::new(x, y)))
            .with(TextureIndex(None))
            .with(Color(Rgba::new(1.0, 1.0, 1.0, 1.0)))
            .with(Tile)
            .build();
    }

    pub fn tick(&mut self) {
        let now = instant::Instant::now();
        let duration = now.duration_since(self.last);
        self.last = now;
        *self.ecs.write_resource::<DeltaTime>() = DeltaTime(duration);
        self.dis.dispatch(&self.ecs);
        self.ecs.maintain();
    }
}
