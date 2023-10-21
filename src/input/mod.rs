mod travel_mode;

use crate::{
    camera::ControlEvent,
    cursor_control::{CursorControl, InputMode},
};
use bevy::{
    input::mouse::{MouseMotion, MouseWheel},
    prelude::*,
};
use bevy_egui::EguiContexts;

/// # Documentation
///
/// Input handling: https://bevy-cheatbook.github.io/builtins.html#input-handling-resources
/// Input event list: https://bevy-cheatbook.github.io/builtins.html#input-events
pub fn handle_mouse(
    contexts: EguiContexts,
    btn_state: Res<Input<MouseButton>>,
    mut mouse_motions: EventReader<MouseMotion>,
    mut mouse_wheels: EventReader<MouseWheel>,
    mut view_evs: EventWriter<ControlEvent>,
) {
    // If the cursor is on top of egui, do not create control events
    if egui_is_hovered(contexts) {
        return;
    }

    handle_drag_events(btn_state, &mut mouse_motions, &mut view_evs);

    // Handle scroll events
    handle_wheel_events(&mut mouse_wheels, &mut view_evs);
}

fn egui_is_hovered(mut contexts: EguiContexts) -> bool {
    let ctx = contexts.ctx_mut();
    ctx.is_pointer_over_area()
}

fn handle_wheel_events(
    mouse_wheels: &mut EventReader<MouseWheel>,
    view_evs: &mut EventWriter<ControlEvent>,
) {
    for ev in mouse_wheels.iter() {
        let MouseWheel { unit: _unit, y, .. } = ev;
        /*match unit {
            MouseScrollUnit::Line => todo!(),
            MouseScrollUnit::Pixel => todo!(),
        }*/
        view_evs.send(ControlEvent::ZoomIn(*y));
    }
}

fn handle_drag_events(
    btn_state: Res<Input<MouseButton>>,
    mouse_motions: &mut EventReader<MouseMotion>,
    view_evs: &mut EventWriter<ControlEvent>,
) {
    let lmb_pressed = btn_state.pressed(MouseButton::Left);
    if lmb_pressed {
        for motion in mouse_motions.iter() {
            view_evs.send(ControlEvent::Pan(-motion.delta))
        }
    }
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
    contexts: EguiContexts,
    time: Res<Time>,
    skeyboard: Res<Input<KeyCode>>,
    view_moves: EventWriter<ControlEvent>,
    control: Res<CursorControl>,
) {
    match control.input_mode {
        InputMode::Travel => {
            travel_mode::handle_keyboard(contexts, time, control, skeyboard, view_moves)
        }
        InputMode::Edit(_) => todo!(),
    }
}
