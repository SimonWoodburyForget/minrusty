use shrev::*;
use specs::prelude::*;
use std::convert::TryInto;
use vek::*;

use winit::event::{ElementState, KeyboardInput, VirtualKeyCode};

/// Represents the direction the player wants to go.
#[derive(Default, Debug, Clone, Copy)]
pub struct InputState {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool,
    pub cursor: Vec2<u32>,
}

impl InputState {
    fn apply(&mut self, event: Event) {
        let Self {
            up,
            down,
            left,
            right,
            cursor,
        } = self;

        match event {
            Event::Up(p) => *up = p,
            Event::Down(p) => *down = p,
            Event::Left(p) => *left = p,
            Event::Right(p) => *right = p,
            Event::Cursor(position) => *cursor = position,
            _ => {}
        };
    }
}

#[derive(Copy, Clone)]
pub enum Event {
    Up(bool),
    Down(bool),
    Left(bool),
    Right(bool),
    Cursor(Vec2<u32>),
    Unknown,
}

impl From<KeyboardInput> for Event {
    fn from(event: KeyboardInput) -> Self {
        let KeyboardInput {
            virtual_keycode,
            state,
            ..
        } = event;

        let held = ElementState::Pressed == state;

        if let Some(vkc) = virtual_keycode {
            match vkc {
                VirtualKeyCode::Up => Event::Up(held),
                VirtualKeyCode::Down => Event::Down(held),
                VirtualKeyCode::Left => Event::Left(held),
                VirtualKeyCode::Right => Event::Right(held),
                _ => Event::Unknown,
            }
        } else {
            Event::Unknown
        }
    }
}

impl From<crate::CursorInput> for Event {
    fn from(event: crate::CursorInput) -> Self {
        let crate::CursorInput(position) = event;
        let [x, y] = position.into_array();
        // NOTE: can cursor positions actually be negative?
        Event::Cursor(Vec2::new(x.try_into().unwrap(), y.try_into().unwrap()))
    }
}

impl From<InputState> for Vec2<f32> {
    /// Converts InputState into a unit vector, (Vec2 with a lenght of 1) if a button
    /// is pressed otherwise it returns zero vector.
    fn from(state: InputState) -> Self {
        let to_float = |x| if x { 1.0 } else { 0.0 };

        #[rustfmt::skip]
        let InputState { up, down, left, right, .. } = state;
        Vec2::new(
            to_float(right) - to_float(left),
            to_float(up) - to_float(down),
        )
        .try_normalized()
        .unwrap_or(Vec2::zero())
    }
}

#[derive(Default)]
pub struct InputSystem {
    reader_id: Option<ReaderId<Event>>,
}

impl<'a> System<'a> for InputSystem {
    type SystemData = (Read<'a, EventChannel<Event>>, Write<'a, InputState>);

    fn run(&mut self, (inputs, mut state): Self::SystemData) {
        for event in inputs.read(&mut self.reader_id.as_mut().unwrap()) {
            state.apply(*event);
        }
    }

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);

        self.reader_id = Some(world.fetch_mut::<EventChannel<Event>>().register_reader());
    }
}
