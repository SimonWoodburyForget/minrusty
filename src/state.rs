use crate::components::*;
use crate::input::*;
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
    pub fn new() -> Self {
        let mut world = World::new();

        world.register::<Position>();
        world.register::<Identity>();
        world.register::<Tile>();
        world.register::<RenderId>();

        world.insert(GameStart::default());
        world.insert(DeltaTime::default());

        let mut dispatcher = DispatcherBuilder::new()
            .with(InputSystem(None), "input-system", &[])
            .with(PlayerSystem, "player-system", &[])
            .with(PhysicSystem, "physic-system", &[])
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
            .with(RenderId(None))
            .build();
    }

    pub fn create_player(&mut self) {
        self.ecs
            .create_entity()
            .with(Position(Vec3::new(0.0, 0.0, 2.0)))
            .with(Velocity(Vec2::zero()))
            .with(Control)
            .with(RenderId(None))
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
