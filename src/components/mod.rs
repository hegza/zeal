use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

/// Physical state
#[derive(Component)]
pub struct BubblePhysics {
    /// Position
    pub pos: Vec2,
    /// Velocity
    pub vel: Vec2,
}
