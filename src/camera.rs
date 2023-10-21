use bevy::prelude::*;

use crate::cursor_control::{CursorControl, InputMode};

#[derive(Component)]
pub struct MainCamera;

#[derive(Event, Debug, Clone)]
pub enum ControlEvent {
    Pan(Vec2),
    ZoomIn(f32),
    ChangeMode(InputMode),
}

const MIN_SCALE: f32 = 0.2;

pub fn handle_view_event(
    mut view_moves: EventReader<ControlEvent>,
    mut q: Query<&mut OrthographicProjection, With<MainCamera>>,
    mut control: ResMut<CursorControl>,
) {
    for motion in view_moves.iter() {
        let mut projection = q.single_mut();
        match motion {
            ControlEvent::Pan(xy) => handle_pan(&mut projection, xy),
            ControlEvent::ZoomIn(amount) => handle_zoom_in(&mut projection, *amount),
            ControlEvent::ChangeMode(nmode) => handle_change_mode(&mut control, nmode.clone()),
        }
    }
}

fn handle_change_mode(control: &mut CursorControl, nmode: InputMode) {
    control.input_mode = nmode;
}

fn handle_zoom_in(projection: &mut OrthographicProjection, amount: f32) {
    projection.scale = (projection.scale - amount).max(MIN_SCALE);
}

fn handle_pan(projection: &mut OrthographicProjection, xy: &Vec2) {
    let a = &projection.area;
    let pan = Vec2::new(-xy.x / a.width(), xy.y / a.height()) * projection.scale;
    projection.viewport_origin += pan;
}
