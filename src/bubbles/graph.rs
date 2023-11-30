use bevy::prelude::*;
use petgraph::stable_graph::StableDiGraph;

pub type BubbleId = u32;

#[derive(Resource, Default)]
pub struct BubbleGraph {
    graph: StableDiGraph<BubbleId, ()>,
}

#[derive(Debug)]
pub enum BubbleGraphError {
    NotPresent,
}

impl BubbleGraph {
    pub fn insert(&mut self) -> BubbleId {
        let uuid = self.graph.add_node(0);
        uuid.index() as BubbleId
    }

    pub fn remove(&mut self, id: BubbleId) -> Option<u32> {
        self.graph.remove_node(id.into())
    }

    /// Returns error if either node did not exist
    pub fn add_edge(&mut self, left: BubbleId, right: BubbleId) -> Result<(), BubbleGraphError> {
        self.graph.add_edge(left.into(), right.into(), ());

        Ok(())
    }

    pub fn remove_edge(&mut self, id: BubbleId) -> Option<()> {
        self.graph.remove_edge(id.into())
    }

    pub fn neighbors(&self, idx: BubbleId) -> Vec<BubbleId> {
        self.graph
            .neighbors(idx.into())
            .into_iter()
            .map(|x| x.index() as u32)
            .collect()
    }

    pub fn contains_node(&self, idx: BubbleId) -> bool {
        self.graph.contains_node(idx.into())
    }
}
