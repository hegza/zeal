use bevy::{prelude::*, time::Time};
use bevy_egui::EguiContexts;

use crate::{
    camera::ControlEvent,
    cursor_control::{CursorControl, InputMode},
};

/// # Documentation
///
/// Input handling: https://bevy-cheatbook.github.io/builtins.html#input-handling-resources
/// Input event list: https://bevy-cheatbook.github.io/builtins.html#input-events
///
/// # Arguments
///
/// * `skeyboard` - Keyboard state
pub fn handle_keyboard(
    mut contexts: EguiContexts,
    time: Res<Time>,
    control: Res<CursorControl>,
    keyboard_state: Res<Input<KeyCode>>,
    mut control_events: EventWriter<ControlEvent>,
) {
    // If egui wants keyboard input, do not create view events
    // TODO: this is not ideal for all cases -> refactor later
    let ctx = contexts.ctx_mut();
    if ctx.wants_keyboard_input() {
        return;
    }

    handle_arrow_keys(&keyboard_state, &time, &mut control_events);

    if keyboard_state.pressed(KeyCode::I) {
        if let Some(bubble_id) = control.selected {
            // Change from travel mode to edit mode
            control_events.send(ControlEvent::ChangeMode(InputMode::Edit(bubble_id)));
        }
    }
}

fn handle_arrow_keys(
    keyboard_state: &Input<KeyCode>,
    time: &Time,
    control_events: &mut EventWriter<ControlEvent>,
) {
    if let Some(v) = arrow_keys_to_vec(&keyboard_state) {
        const KB_MOVE_PX_PER_SEC: f32 = 500.;
        let pan_event = ControlEvent::Pan(v * KB_MOVE_PX_PER_SEC * time.delta_seconds());
        control_events.send(pan_event);
    }
}

/// Convert arrow keys (left, right, up down) into a normalized vector such as `(1., 0.)` for right arrow or `(-1.,
/// -1.)` for up and left at the same time
fn arrow_keys_to_vec(skeyboard: &Input<KeyCode>) -> Option<Vec2> {
    let left = skeyboard.pressed(KeyCode::Left);
    let right = skeyboard.pressed(KeyCode::Right);
    let up = skeyboard.pressed(KeyCode::Up);
    let down = skeyboard.pressed(KeyCode::Down);

    let dx = match (left, right) {
        (true, false) => Some(-1.),
        (false, true) => Some(1.),
        _ => None,
    };
    let dy = match (up, down) {
        (true, false) => Some(-1.),
        (false, true) => Some(1.),
        _ => None,
    };

    if dx.is_none() && dy.is_none() {
        return None;
    }

    let x = dx.unwrap_or(0.);
    let y = dy.unwrap_or(0.);
    Some(Vec2::new(x, y))
}
