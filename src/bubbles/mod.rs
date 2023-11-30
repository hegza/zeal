mod graph;
mod visuals;

pub use graph::BubbleId;
pub use visuals::BubbleBundleBuilder;

use self::graph::BubbleGraphError;
use crate::physics::{Force, GlobalPhysics};
use bevy::prelude::*;
use petgraph::stable_graph::IndexType;
use rand::{thread_rng, Rng};
// Do not expose graph::BubbleGraph. It is used through the `Bubbles` interface
// that manages also the render graph.
use graph::BubbleGraph;

#[derive(Resource, Default)]
pub struct Bubbles {
    graph: BubbleGraph,
}

impl Bubbles {
    /// Spawns a bubble as a child to the given `parent`
    ///
    /// Bubble's position depends on the other bubbles connected to the same parent.
    ///
    /// Returns error if the parent didn't exist.
    pub fn spawn_child(
        &mut self,
        parent: BubbleId,
        render_graph: &mut BubbleBundleBuilder,
        physics: &mut GlobalPhysics,
    ) -> Result<(), BubbleGraphError> {
        let parent_pos = render_graph.position(parent);
        let mut rng = thread_rng();
        let pos = below(parent_pos, SPAWN_DIST) + Vec2::X * (50. * rng.gen::<f32>() - 25.);
        let child = self.spawn_orphan(pos, render_graph);

        assert!(self.graph.contains_node(parent));
        assert!(self.graph.contains_node(child));
        self.graph.add_edge(child, parent)?;
        render_graph.connect(child, parent);

        physics.add_force(Force::Explosion {
            origin: pos,
            force: BUBBLE_SPAWN_FORCE,
            r: BUBBLE_SPAWN_FORCE_DIST,
        });
        Ok(())
    }

    /// Spawns an orphan bubble at given position
    pub fn spawn_orphan(&mut self, pos: Vec2, render_graph: &mut BubbleBundleBuilder) -> BubbleId {
        let id = self.graph.insert();
        render_graph.create_bubble(id, pos);
        id
    }

    pub fn neighbors(&self, idx: BubbleId) -> Vec<BubbleId> {
        self.graph
            .neighbors(idx.into())
            .into_iter()
            .map(|x| x.index() as u32)
            .collect()
    }
}

const SPAWN_DIST: f32 = 100.;
const BUBBLE_SPAWN_FORCE_DIST: f32 = 50.;
const BUBBLE_SPAWN_FORCE: f32 = 0.00001;

fn below(origin: Vec2, dist: f32) -> Vec2 {
    origin + Vec2::NEG_Y * dist
}
