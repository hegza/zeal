use bevy::prelude::*;

pub const DEFAULT_FCENTER: f32 = 3.;
pub const DEFAULT_SLOW_MULT: f32 = 10.;
pub const DEFAULT_FREPEL: f32 = 10_000_000.;

#[derive(Resource)]
pub struct GlobalPhysics {
    /// Centering force
    ///
    /// Each bubble is accelerated towards [0, 0] at `fcenter` per second
    pub fcenter: f32,
    /// The speed of each bubble is multiplied by `speed_mult` per second
    pub slow_mult: f32,
    /// Repel force between bubble
    ///
    /// TODO: should apply based on distance between bubble edges, not centers
    pub frepel: f32,
}

impl Default for GlobalPhysics {
    fn default() -> Self {
        // Configure physics

        let fcenter = DEFAULT_FCENTER;
        let slow_mult = DEFAULT_SLOW_MULT;
        let frepel = DEFAULT_FREPEL;

        Self {
            fcenter,
            slow_mult,
            frepel,
        }
    }
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
    let positions = q.iter().map(|bubble| bubble.pos).collect::<Vec<_>>();
    for (bidx, mut bubble) in q.iter_mut().enumerate() {
        // Apply centering force
        let fcenter = -bubble.pos * gphysics.fcenter;
        bubble.vel += fcenter * time.delta_seconds();

        // Apply repel
        // TODO: optimize by separating to another system and filtering based on distance
        let one_div_by_distances_squared: Vec2 = positions
            .iter()
            .enumerate()
            // Do not repel self
            .filter(|(oidx, _)| bidx != *oidx)
            .map(|(_, opos)| {
                let diff = bubble.pos - *opos;
                let rdist = 1. / (diff.length() * diff.length());
                let unit = diff.normalize();
                unit * rdist
            })
            .sum();
        bubble.vel += one_div_by_distances_squared * gphysics.frepel * time.delta_seconds();

        // Apply slow down
        // TODO: find a good formula
        bubble.vel *= 1. - gphysics.slow_mult * time.delta_seconds();

        // Apply velocity
        let delta = bubble.vel * time.delta_seconds();
        bubble.pos += delta;
    }
}
