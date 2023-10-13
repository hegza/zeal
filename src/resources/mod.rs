use bevy::prelude::Resource;

#[derive(Default, Resource)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

#[derive(Default, Resource)]
pub enum InputMode {
    #[default]
    Travel,
    Insert,
}

impl InputMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            InputMode::Travel => "travel",
            InputMode::Insert => "insert",
        }
    }
}

#[derive(Resource, Default)]
pub struct GlobalPhysics {
    /// Centering force
    ///
    /// Each bubble is accelerated towards [0, 0] at `fcenter` per second
    pub fcenter: f32,
}
