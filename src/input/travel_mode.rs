use bevy::{prelude::*, time::Time};
use bevy_egui::EguiContexts;

use crate::camera::ControlEvent;

use super::arrow_keys_to_vec;

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
