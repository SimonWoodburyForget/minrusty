use crate::components::*;
use instant;
use specs::prelude::*;
use std::borrow::Cow;
use vek::{Vec2, Vec3};

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

/// Game render resource.
#[derive(Default)]
pub struct GameRender {
    /// Seconds feed through a sin function.
    pub sin_wave: f32,
}

/// Game rendering system. Turns resources and components into something that
/// can actually be rendered by the OpenGL pipeline.
pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Identity>,
        ReadStorage<'a, Position>,
        ReadStorage<'a, Tile>,
        Read<'a, GameStart>,
        Write<'a, GameRender>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (entities, ident, pos, tile, start, mut render) = data;

        // (entities, ident, pos, tile)
        //     .join()
        //     .for_each(|(entity, ident, pos, _)| {
        //         // .. do stuff
        //     });

        let elapsed = start.0.elapsed();
        let sec_from_start = elapsed.as_secs() as f32 + elapsed.subsec_nanos() as f32 * 1e-9;

        use f32;
        *render = GameRender {
            sin_wave: sec_from_start.sin(),
        }
    }
}

pub struct GameState {
    pub ecs: World,
    dis: Dispatcher<'static, 'static>,

    /// Last update instant.
    last: instant::Instant,
}

impl GameState {
    pub fn new() -> Self {
        let mut world = World::new();
        world.insert(DeltaTime::default());

        let mut dispatcher = DispatcherBuilder::new()
            .with(RenderSystem, "render-system", &[])
            .build();
        dispatcher.setup(&mut world);

        Self {
            ecs: world,
            dis: dispatcher,
            last: instant::Instant::now(),
        }
    }

    pub fn create_block<'a, T>(&mut self, x: f32, y: f32, name: T)
    where
        T: Into<Cow<'a, str>>,
    {
        self.ecs
            .create_entity()
            .with(Identity(name.into().to_string()))
            .with(Position(Vec3::new(x, y, 1.0)))
            .with(Tile)
            .build();
    }

    pub fn update(&mut self) {
        let now = instant::Instant::now();
        let duration = now.duration_since(self.last);
        {
            let mut dt = self.ecs.write_resource::<DeltaTime>();
            *dt = DeltaTime(duration);
        }
        self.last = now;

        self.dis.dispatch(&self.ecs);
        self.ecs.maintain();
    }
}
