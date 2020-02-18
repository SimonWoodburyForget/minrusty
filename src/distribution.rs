use crate::components::*;

use specs::prelude::*;
use std::collections::BTreeMap;
use vek::*;

struct Node {
    coordinate: Vec2<u32>,
    storage: [bool; 4],
    destination: Vec2<u32>,
}

#[derive(Clone, Copy)]
struct Edge(usize, Option<usize>);

/// Represents distribution as a graph.
#[derive(Default)]
struct DistVolume {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    node_map: BTreeMap<[u32; 2], usize>,
}

impl DistVolume {
    fn insert_conveyor(&mut self, coordinate: Vec2<u32>, destination: Vec2<u32>) {
        let [x, y] = coordinate.into_array();
        let Self {
            nodes,
            edges,
            node_map,
        } = self;

        // push in new node
        let node_index = nodes.len();
        nodes.push(Node {
            coordinate,
            storage: Default::default(),
            destination,
        });

        // find neighbours of new node
        let neighbours = [
            node_map.get(&[x + 1, y]).map(|x| *x),
            node_map.get(&[x - 1, y]).map(|x| *x),
            node_map.get(&[x, y + 1]).map(|x| *x),
            node_map.get(&[x, y - 1]).map(|x| *x),
        ];

        for neighbour_node_index in neighbours.iter().filter_map(|x| *x) {
            let mut neighbour_edge = edges
                .iter_mut()
                .find(|some_index| some_index.0 == neighbour_node_index)
                .unwrap();
            neighbour_edge.1 = Some(node_index);
        }

        // attach new node to neighbour
        node_map.insert(coordinate.into_array(), node_index);
        let node_index_other = node_map.get(&[x, y]).map(|x| *x);

        edges.push(Edge(node_index, node_index_other));
    }
}

#[derive(Default)]
pub struct DistSystem {
    /// Cached state of the distribution network in a graph structure.
    volume: DistVolume,

    conv_rid: Option<ReaderId<ComponentEvent>>,
    inserted: BitSet,
    modified: BitSet,
    removed: BitSet,
}

impl<'a> System<'a> for DistSystem {
    type SystemData = (
        ReadStorage<'a, Coordinate>,
        ReadStorage<'a, Conveyor>,
        ReadStorage<'a, ItemDestination>,
        // .. WriteStorage<'a, Item>,
    );

    fn setup(&mut self, world: &mut World) {
        Self::SystemData::setup(world);
        self.conv_rid = Some(world.write_storage::<Conveyor>().register_reader());
    }

    fn run(&mut self, (coord, convs, dests): Self::SystemData) {
        let Self {
            volume,

            modified,
            inserted,
            removed,
            conv_rid,
        } = self;

        modified.clear();
        inserted.clear();
        removed.clear();

        for event in convs.channel().read(&mut conv_rid.as_mut().unwrap()) {
            use ComponentEvent::{Inserted, Modified, Removed};
            match event {
                Modified(id) => modified.add(*id),
                Inserted(id) => inserted.add(*id),
                Removed(id) => removed.add(*id),
            };
        }

        // TODO: with tracked conveyors, build a graph of them to move items around.

        for (coord, dest, conveyor_id, _) in (&coord, &dests, &convs, &self.inserted).join() {
            let v = Vec2::new(coord.0.x as u32, coord.0.y as u32);
            let id = volume.insert_conveyor(v, dest.0);
            // TODO: ...
        }
    }
}
