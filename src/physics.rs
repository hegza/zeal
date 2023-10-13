use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GlobalPhysics {
    /// Centering force
    ///
    /// Each bubble is accelerated towards [0, 0] at `fcenter` per second
    pub fcenter: f32,
}

/// Physical state
#[derive(Component)]
pub struct BubblePhysics {
    /// Position
    pub pos: Vec2,
    /// Velocity
    pub vel: Vec2,
}

pub fn bubble_physics(
    time: Res<Time>,
    gphysics: Res<GlobalPhysics>,
    mut q: Query<&mut BubblePhysics>,
) {
    for mut bubble in q.iter_mut() {
        // Apply centering force
        let fcenter = -bubble.pos * gphysics.fcenter;
        bubble.vel += fcenter * time.delta_seconds();

        // Apply velocity
        let delta = bubble.vel * time.delta_seconds();
        bubble.pos += delta;
    }
}
