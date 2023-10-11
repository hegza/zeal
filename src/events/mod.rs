use bevy::prelude::*;

#[derive(Event)]
pub struct ViewMoveEvent {
    pub x: f32,
    pub y: f32,
}

impl ViewMoveEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
