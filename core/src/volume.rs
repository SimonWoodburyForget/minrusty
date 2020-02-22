use tinyvec::ArrayVec;

/// Size of a chunk is 32 by 32 for no specific reason.
const ChunkSize: usize = 32 * 32;

trait Voxel: Default + Copy {}

/// Type held by a chunk which points to another chunk.
#[derive(Default)]
struct Edge {
    /// Index of the neighbour vertex.
    vertex_index: u32,
    // TODO: more edge data?
}

struct Chunk<T: Voxel> {
    /// Position of the chunk in world coordinates.
    position: [u32; 2],

    /// A chunk of tile data.
    tiles: [T; ChunkSize],

    /// Stores neighbours of the nodes (by index) with a maximum of 8 neighbours.
    chunk_neighbours: ArrayVec<[Edge; 8]>,
}

impl<T: Voxel> Default for Chunk<T> {
    fn default() -> Self {
        let (position, chunk_neighbours) = Default::default();
        Self {
            position,
            chunk_neighbours,
            tiles: [Default::default(); ChunkSize],
        }
    }
}

/// Theoretically
#[derive(Default)]
struct GraphChunkVolume<T: Voxel> {
    /// Chunks are nodes.
    chunks: Vec<Chunk<T>>,
}

impl<T: Voxel> GraphChunkVolume<T> {
    fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn getter_setter() {
        impl Voxel for bool {}
        let vol = GraphChunkVolume::<bool>::new();
    }
}
