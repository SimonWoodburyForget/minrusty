use crate::components::*;
use crate::input::*;
use crate::loader::*;
use crate::physics::*;
use crate::player::*;

use instant;
use specs::prelude::*;
use std::borrow::Cow;
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

        world.register::<Position>();
        world.register::<Name>();
        world.register::<Tile>();
        world.register::<RenderId>();
        world.register::<Coordinate>();
        world.register::<TextureIndex>();

        world.insert(GameStart::default());
        world.insert(DeltaTime::default());
        // world.insert(AssetSystem::default());

        let mut dispatcher = DispatcherBuilder::new()
            .with(AssetSystem::default(), "asset-system", &[])
            .with(InputSystem(None), "input-system", &[])
            .with(PlayerSystem, "player-system", &["input-system"])
            .with(PhysicSystem, "physic-system", &["player-system"])
            .with_thread_local(renderer)
            .build();
        dispatcher.setup(&mut world);

        Self {
            ecs: world,
            dis: dispatcher,
            last: instant::Instant::now(),
        }
    }

    pub fn create_block(&mut self, x: usize, y: usize, name: &str) {
        self.ecs
            .create_entity()
            .with(Name(name.into()))
            .with(Coordinate(Vec2::new(x, y)))
            .with(Tile)
            .with(RenderId(None))
            .with(TextureIndex(None))
            .build();
    }

    pub fn create_player(&mut self) {
        self.ecs
            .create_entity()
            .with(Position(Vec3::new(0.0, 0.0, -0.5)))
            .with(Velocity(Vec2::zero()))
            .with(Force(Vec2::zero()))
            .with(Control)
            .with(RenderId(None))
            .build();
    }

    pub fn update(&mut self) {
        let now = instant::Instant::now();
        let duration = now.duration_since(self.last);
        self.last = now;
        {
            let mut dt = self.ecs.write_resource::<DeltaTime>();
            *dt = DeltaTime(duration);
        }

        self.dis.dispatch(&self.ecs);
        self.ecs.maintain();
    }
}
