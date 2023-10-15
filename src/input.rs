use crate::camera::ViewEvent;
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};

/// # Documentation
///
/// Input handling: https://bevy-cheatbook.github.io/builtins.html#input-handling-resources
/// Input event list: https://bevy-cheatbook.github.io/builtins.html#input-events
pub fn handle_mouse(
    btn_state: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut mouse_scroll: EventReader<MouseWheel>,
    mut view_moves: EventWriter<ViewEvent>,
) {
    handle_drag_events(btn_state, &mut mouse_motion, &mut view_moves);

    // Handle scroll events
    for ev in mouse_scroll.iter() {
        let MouseWheel { unit: _unit, y, .. } = ev;
        /*match unit {
            MouseScrollUnit::Line => todo!(),
            MouseScrollUnit::Pixel => todo!(),
        }*/
        view_moves.send(ViewEvent::ZoomIn(*y));
    }
}

fn handle_drag_events(
    btn_state: Res<Input<MouseButton>>,
    mouse_motion: &mut EventReader<MouseMotion>,
    view_moves: &mut EventWriter<ViewEvent>,
) {
    let lmb_pressed = btn_state.pressed(MouseButton::Left);
    if lmb_pressed {
        for motion in mouse_motion.iter() {
            view_moves.send(ViewEvent::Pan(-motion.delta))
        }
    }
}

/// Convert arrow keys (left, right, up down) into a normalized vector such as `(1., 0.)` for right arrow or `(-1.,
/// -1.)` for up and left at the same time
fn arrow_keys_to_vec(skeyboard: &Input<KeyCode>) -> Option<Vec2> {
    let left = skeyboard.pressed(KeyCode::Left);
    let right = skeyboard.pressed(KeyCode::Right);
    let up = skeyboard.pressed(KeyCode::Up);
    let down = skeyboard.pressed(KeyCode::Down);

    let mut dx = None;
    let mut dy = None;
    match (left, right) {
        (true, false) => dx = Some(-1.),
        (false, true) => dx = Some(1.),
        _ => {}
    }
    match (up, down) {
        (true, false) => dy = Some(-1.),
        (false, true) => dy = Some(1.),
        _ => {}
    }

    if dx.is_none() && dy.is_none() {
        return None;
    }

    let x = dx.unwrap_or(0.);
    let y = dy.unwrap_or(0.);
    Some(Vec2::new(x, y))
}

/// # Documentation
///
/// Input handling: https://bevy-cheatbook.github.io/builtins.html#input-handling-resources
/// Input event list: https://bevy-cheatbook.github.io/builtins.html#input-events
///
/// # Arguments
///
/// * `skeyboard` - Keyboard state
pub fn handle_keyboard(
    time: Res<Time>,
    skeyboard: Res<Input<KeyCode>>,
    mut view_moves: EventWriter<ViewEvent>,
) {
    if let Some(v) = arrow_keys_to_vec(&skeyboard) {
        const KB_MOVE_PX_PER_SEC: f32 = 500.;
        view_moves.send(ViewEvent::Pan(
            v * KB_MOVE_PX_PER_SEC * time.delta_seconds(),
        ));
    }
}
