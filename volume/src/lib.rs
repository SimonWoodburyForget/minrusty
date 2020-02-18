use std::collections::BTreeMap;

/// Chunk size of 32 results in 1024 size chunks.
const CHUNK_SIZE: usize = 32;

type Chunk<T> = [T; CHUNK_SIZE * CHUNK_SIZE];

/// Spacial map of statically sized 32 by 32 chunks.
pub struct SpacialMap<T> {
    chunks: BTreeMap<[u32; 2], Chunk<T>>,
}

use index::Index;
mod index {
    /// Internal `Index` type for SpacialMap.
    pub struct Index {
        pub xy: ChunkIndex,
        pub n: TileIndex,
    }

    type ChunkIndex = [u32; 2];
    type TileIndex = usize;

    impl From<[u32; 2]> for Index {
        fn from(x: [u32; 2]) -> Self {
            chunkmap_index(x)
        }
    }
    
    fn chunkmap_index(position: [u32; 2]) -> Index {
        let [x, y] = position;
        let cx = x % super::CHUNK_SIZE as u32;
        let cy = y % super::CHUNK_SIZE as u32;
        let n = (x - cx) * (y - cy);
        Index {
            xy: [cx, cy],
            n: n as usize
        }
    }
}

impl<T> SpacialMap<T> {
    /// Get's a tile.
    pub fn get<P: Into<Index>>(&self, xy: P) -> Option<&T> {
        let Index { xy, n } = xy.into();
        self.chunks.get(&xy).map(|c| &c[n])
    }

    pub fn set<P: Into<Index>>(&mut self, xy: P, v: T) {
        let Index { xy, n } = xy.into();

        if let Some(chunk) = self.chunks.get_mut(&xy) {
            chunk[n] = v;
        }
    }
}
