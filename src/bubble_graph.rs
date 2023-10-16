use bevy::prelude::Resource;
use petgraph::stable_graph::{Neighbors, StableUnGraph};

pub type BubbleId = u32;

#[derive(Resource, Default)]
pub struct BubbleGraph {
    graph: StableUnGraph<BubbleId, BubbleId>,
    max_idx: BubbleId,
}

/*{
    /// Connections
    conns: HashMap<BubbleId, Vec<BubbleId>>,
}*/

#[derive(Debug)]
pub enum BubbleGraphError {
    NotPresent,
}

impl BubbleGraph {
    pub fn add_bubble(&mut self) -> BubbleId {
        let uuid = self.max_idx;
        self.max_idx += 1;
        self.graph.add_node(self.max_idx);
        uuid
    }

    /// Returns error if either node did not exist
    pub fn connect(&mut self, left: BubbleId, right: BubbleId) -> Result<(), BubbleGraphError> {
        self.graph.add_edge(left.into(), right.into(), 1);

        /*
        let [left_targets, right_targets] = self
            .conns
            .get_many_mut([left, right])
            .ok_or(BubbleGraphError::NotPresent)?;
        left_targets.push(*right);
        right_targets.push(*left);
        */
        Ok(())
    }

    pub fn neighbors(&self, idx: BubbleId) -> Vec<BubbleId> {
        self.graph
            .neighbors(idx.into())
            .into_iter()
            .map(|x| x.index() as u32)
            .collect()
    }
}

/*
impl BubbleGraph {
    /*
    pub fn create_bubble(&mut self) -> BubbleId {
        let bid = Uuid::new_v4();
        self.conns.insert(bid, Vec::with_capacity(4));
        bid
    }

    /// Returns whether the value was present
    pub fn remove_bubble(&mut self, id: BubbleId) -> bool {
        let present = self.conns.remove(&id).is_some();

        // Also remove connections in the other direction (where target is this)
        for (_, targets) in &mut self.conns {
            if let Some(t) = targets.iter().position(|t| t == &id) {
                targets.swap_remove(t);
            }
        }
        present
    }

    pub fn disconnect(
        &mut self,
        left: &BubbleId,
        right: &BubbleId,
    ) -> Result<(), BubbleGraphError> {
        let [left_targets, right_targets] = self
            .conns
            .get_many_mut([left, right])
            .ok_or(BubbleGraphError::NotPresent)?;
        left_targets.swap_remove(*right);
        right_targets.swap_remove(*left);
        Ok(())
    }
    */
}
*/
