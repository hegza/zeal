use std::mem;

use bevy::prelude::*;

pub const DEFAULT_FCENTER: f32 = 3.;
pub const DEFAULT_SLOW_MULT: f32 = 10.;
pub const DEFAULT_FREPEL: f32 = 10_000_000.;
pub const DEFAULT_FLINK: f32 = 4.;

#[derive(Resource)]
pub struct GlobalPhysics {
    // TODO: merge all forces into `active_forces`
    /// Centering force
    ///
    /// Each bubble is accelerated towards [0, 0] at `fcenter` per second
    pub fcenter: f32,
    /// The speed of each bubble is multiplied by `speed_mult` per second
    pub slow_mult: f32,
    /// Repel force multiplier between bubbles
    ///
    /// TODO: should apply based on distance between bubble edges, not centers
    pub frepel: f32,
    /// Link pull multiplier per distance (k in kx), i.e., stiffness
    pub flink: f32,
    active_forces: Vec<Force>,
}

impl Default for GlobalPhysics {
    fn default() -> Self {
        // Configure physics

        let fcenter = DEFAULT_FCENTER;
        let slow_mult = DEFAULT_SLOW_MULT;
        let frepel = DEFAULT_FREPEL;
        let flink = DEFAULT_FLINK;

        Self {
            fcenter,
            slow_mult,
            frepel,
            flink,
            active_forces: vec![],
        }
    }
}

impl GlobalPhysics {
    pub fn add_force(&mut self, f: Force) {
        self.active_forces.push(f);
    }

    pub fn update_bubbles(&mut self, dt: f32, mut q: Query<(&mut BubblePhysics, &mut Transform)>) {
        self.update_global_forces(dt, &mut q);
        self.update_repels(dt, &mut q);
    }

    fn update_global_forces(
        &mut self,
        dt: f32,
        q: &mut Query<(&mut BubblePhysics, &mut Transform)>,
    ) {
        for (mut bubble, mut tfm) in q.iter_mut() {
            let pos = Vec2::new(tfm.translation.x, tfm.translation.y);

            // Apply centering force
            let fcenter = -pos * self.fcenter;
            bubble.vel += fcenter * dt;

            // Apply other forces
            for f in &self.active_forces {
                let v = f.effect_on_point(pos);
                bubble.vel += v;
            }

            // Apply slow down
            // TODO: find a good formula
            bubble.vel *= 1. - self.slow_mult * dt;

            // Apply velocity
            let delta = bubble.vel * dt;
            tfm.translation = Vec3::new(
                tfm.translation.x + delta.x,
                tfm.translation.y + delta.y,
                tfm.translation.z,
            );
        }

        // Retain non-explosion forces
        self.active_forces.retain(|f| {
            mem::discriminant(f)
                != mem::discriminant(&Force::Explosion {
                    origin: Vec2::ZERO,
                    force: 0.,
                    r: 0.,
                })
        })
    }

    fn update_repels(&self, dt: f32, q: &mut Query<(&mut BubblePhysics, &mut Transform)>) {
        let positions = q
            .iter()
            .map(|(_, tfm)| Vec2::new(tfm.translation.x, tfm.translation.y))
            .collect::<Vec<_>>();
        for (bidx, (mut bubble, tfm)) in q.iter_mut().enumerate() {
            let pos = Vec2::new(tfm.translation.x, tfm.translation.y);

            // Apply repel
            // TODO: optimize by separating to another system and filtering based on distance
            let one_div_by_distances_squared: Vec2 = positions
                .iter()
                .enumerate()
                // Do not repel self
                .filter(|(oidx, _)| bidx != *oidx)
                .map(|(_, opos)| {
                    let diff = pos - *opos;
                    let rdist = 1. / (diff.length() * diff.length());
                    let unit = diff.normalize();
                    unit * rdist
                })
                .sum();
            bubble.vel += one_div_by_distances_squared * self.frepel * dt;
        }
    }
}

/// Physical state
#[derive(Component, Default)]
pub struct BubblePhysics {
    /// Velocity
    pub vel: Vec2,
}

pub fn physics_system(
    time: Res<Time>,
    mut gphysics: ResMut<GlobalPhysics>,
    q: Query<(&mut BubblePhysics, &mut Transform)>,
) {
    let dt = time.delta_seconds();
    gphysics.update_bubbles(dt, q);
}

#[derive(Clone)]
pub enum Force {
    /// Instant radial force, like an explosion
    ///
    /// Force decreases exponentially with radius
    Explosion {
        origin: Vec2,
        /// Force at origin
        force: f32,
        /// Radius
        r: f32,
    },
}

impl Force {
    fn effect_on_point(&self, pos: Vec2) -> Vec2 {
        match self {
            Force::Explosion { origin, force, r } => {
                let diff = pos - *origin;
                let distance_from_center = diff.length();
                if distance_from_center <= *r {
                    let frac = 1. - distance_from_center / r;
                    let sq = frac * frac;
                    debug_assert!(sq <= 1.0);
                    force * sq * diff.normalize()
                } else {
                    Vec2::ZERO
                }
            }
        }
    }
}
