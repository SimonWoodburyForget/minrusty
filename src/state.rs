use specs::prelude::*;

pub struct GameState {
    esc: World,
    dis: Dispatcher<'static, 'static>,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            esc: World::new(),
            dis: DispatcherBuilder::new().build(),
        }
    }
}
