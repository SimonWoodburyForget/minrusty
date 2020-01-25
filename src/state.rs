use instant;
use specs::prelude::*;

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

/// Game rendering system. Takes care of updating the game render resource.
pub struct RenderSystem;

impl<'a> System<'a> for RenderSystem {
    type SystemData = (Write<'a, GameRender>, Read<'a, GameStart>);

    fn run(&mut self, data: Self::SystemData) {
        let (mut render, start) = data;

        let sec_from_start =
            (start.0.elapsed().as_secs() as f32 + start.0.elapsed().subsec_nanos() as f32 * 1e-9);

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
