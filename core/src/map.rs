use specs::prelude::*;
use std::ops::{Index, IndexMut};
use vek::*;

/// Tile storage.
pub struct Map<C> {
    tiles: Vec<C>,
    width: u32,
    height: u32,
}

impl<C> Index<Vec2<u32>> for Map<C> {
    type Output = C;
    fn index(&self, position: Vec2<u32>) -> &C {
        &self.tiles[((position.y * self.width) + position.x) as usize]
    }
}

impl<C> IndexMut<Vec2<u32>> for Map<C> {
    fn index_mut(&mut self, position: Vec2<u32>) -> &mut C {
        &mut self.tiles[((position.y * self.width) + position.x) as usize]
    }
}

impl<C: Default> Map<C> {
    fn with_dimentions(dimentions: Vec2<u32>) -> Self {
        let [width, height] = dimentions.into_array();
        Self {
            width,
            height,
            tiles: (0..(width * height)).map(|_| Default::default()).collect(),
        }
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
