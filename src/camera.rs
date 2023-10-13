use bevy::prelude::*;

#[derive(Component)]
pub struct MainCamera;

// TODO: use Vec2
#[derive(Event)]
pub struct ViewMoveEvent(Vec2);

impl ViewMoveEvent {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vec2::new(x, y))
    }

    pub fn x(&self) -> f32 {
        self.0.x
    }

    pub fn y(&self) -> f32 {
        self.0.y
    }
}

impl From<Vec2> for ViewMoveEvent {
    fn from(value: Vec2) -> Self {
        Self(value)
    }
}
