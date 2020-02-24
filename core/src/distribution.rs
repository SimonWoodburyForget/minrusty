use crate::components::*;
use crate::map::*;

use specs::prelude::*;
use std::collections::BTreeMap;
use vek::*;

#[derive(Default)]
pub struct DistSystem;

impl<'a> System<'a> for DistSystem {
    type SystemData = (
        Read<'a, Map<Cell>>,
        ReadStorage<'a, Coordinate>,
        // .. WriteStorage<'a, Conveyor>,
        // .. WriteStorage<'a, Item>,
    );

    fn run(&mut self, _: Self::SystemData) {
        // TODO: move items on conveyors
    }
}
