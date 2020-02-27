use crate::components::*;
use specs::prelude::*;
use std::ops::{Index, IndexMut};
use vek::*;

/// Tile storage.
pub struct Map<C> {
    /// Stores the kind of tile at a coordinate.
    tiles: Vec<C>,

    width: i32,
    pub height: i32,
}

impl<C: Default> Default for Map<C> {
    fn default() -> Self {
        Self::with_dimentions(Vec2::new(32, 32))
    }
}

impl<C> Index<Vec2<i32>> for Map<C> {
    type Output = C;
    fn index(&self, position: Vec2<i32>) -> &C {
        &self.tiles[((position.y * self.width) + position.x) as usize]
    }
}

impl<C> IndexMut<Vec2<i32>> for Map<C> {
    fn index_mut(&mut self, position: Vec2<i32>) -> &mut C {
        &mut self.tiles[((position.y * self.width) + position.x) as usize]
    }
}

impl<C: Default> Map<C> {
    fn with_dimentions(dimentions: Vec2<i32>) -> Self {
        let [width, height] = dimentions.into_array();
        Self {
            width,
            height,
            tiles: (0..(width * height)).map(|_| Default::default()).collect(),
        }
    }
}

#[derive(Default)]
pub struct Cell {
    pub tile: Option<Entity>,
}

#[derive(Default)]
pub struct MappingSystem {
    reader_id: Option<ReaderId<ComponentEvent>>,
    inserted: BitSet,
    modified: BitSet,
    removed: BitSet,
}

impl<'a> System<'a> for MappingSystem {
    type SystemData = (
        Entities<'a>,
        Write<'a, Map<Cell>>,
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Tile>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.reader_id = Some(world.write_storage::<Tile>().register_reader());
    }

    fn run(&mut self, (entities, mut map, coordinates, tiles): Self::SystemData) {
        let Self {
            reader_id,
            modified,
            inserted,
            removed,
        } = self;

        modified.clear();
        inserted.clear();
        removed.clear();

        for event in tiles.channel().read(&mut reader_id.as_mut().unwrap()) {
            use ComponentEvent::{Inserted, Modified, Removed};
            let _ = match event {
                Modified(id) => modified.add(*id),
                Inserted(id) => inserted.add(*id),
                Removed(id) => removed.add(*id),
            };
        }

        for (entity, coordinate, _) in (&entities, &coordinates, &*inserted).join() {
            map[coordinate.0].tile = Some(entity);
        }

        for (entity, coordinate, _) in (&entities, &coordinates, &*modified).join() {
            map[coordinate.0].tile = Some(entity);
        }

        // TODO: tracking removals...?
        // for (entity, coordinate, _) in (&entities, &coordinates, &*removed).join() {
        //     map[coordinate.0].tile = None;
        // }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_test() {
        let mut map: Map<bool> = Map::with_dimentions(Vec2::new(10, 10));
        let index = Vec2::new(2, 3);
        map[index] = true;
        assert!(map[index]);
        assert!(!map[index + Vec2::new(0, 1)]);
    }
}
