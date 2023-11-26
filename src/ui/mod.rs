mod physics_config;

use self::physics_config::physics_config_ui;
use crate::{
    camera::ControlEvent,
    cursor_control::{CursorControl, InputMode},
    physics::GlobalPhysics,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContexts,
};
use std::collections::VecDeque;

#[derive(Resource, Default)]
pub struct OccupiedScreenSpace {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}

/// Capitalizes the first character in s.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn ui_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    cursor_control: Res<CursorControl>,
    mut gphysics: ResMut<GlobalPhysics>,
    history: Res<ControlHistory>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = left_panel(ctx, &cursor_control.input_mode);
    occupied_screen_space.right = right_panel(ctx, &mut gphysics, &history);
    occupied_screen_space.top = top_panel(ctx);
    occupied_screen_space.bottom = bottom_panel(ctx);
}

fn left_panel(ctx: &mut egui::Context, input_mode: &InputMode) -> f32 {
    egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            ui.label(format!("Input mode: {}", capitalize(input_mode.as_str())));
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width()
}

fn right_panel(
    ctx: &mut egui::Context,
    gphysics: &mut GlobalPhysics,
    history: &ControlHistory,
) -> f32 {
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    physics_config_ui(ui, gphysics);
                    input_event_log_ui(ui, history);
                })
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width()
}

fn top_panel(ctx: &mut egui::Context) -> f32 {
    egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Top resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height()
}

fn bottom_panel(ctx: &mut egui::Context) -> f32 {
    egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height()
}

#[derive(Resource)]
pub struct ControlHistory {
    // TODO: an actual cyclic buffer may be more efficient
    queue: VecDeque<ControlEvent>,
    max_len: usize,
}

impl Default for ControlHistory {
    fn default() -> Self {
        Self::with_capacity(128)
    }
}

impl ControlHistory {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            queue: VecDeque::with_capacity(capacity),
            max_len: capacity,
        }
    }
    pub fn extend(&mut self, events: impl Iterator<Item = ControlEvent> + Clone) {
        let push_count = events.clone().count();
        self.extend_with_len(events, push_count);
    }
    pub fn extend_with_len(&mut self, events: impl Iterator<Item = ControlEvent>, len: usize) {
        let space_left_after_push: isize =
            self.max_len as isize - (self.queue.len() + len) as isize;
        if space_left_after_push < 0 {
            let overflow = (-space_left_after_push) as usize;
            // Drain N elements from the front where
            // N = overflow
            self.queue.drain(0..overflow);
        }
        // Add the new events
        self.queue.extend(events);
    }
}

pub fn input_event_log_ui(ui: &mut egui::Ui, history: &ControlHistory) {
    CollapsingHeader::new(format!("Input events (hist = {})", history.queue.len()))
        .default_open(true)
        .show(ui, |ui| {
            let events_it = history.queue.iter().rev();
            let display_vec = merge_similar_events(events_it);
            labels_ui(display_vec.into_iter().map(|x| format!("{x:?}")), ui);
        });
}

/// Merges a list of events by collating events of the same type into single events
fn merge_similar_events<'a>(
    it: impl Iterator<Item = &'a ControlEvent> + Clone,
) -> Vec<ControlEvent> {
    use ControlEvent as CE;

    let mut ret = Vec::with_capacity(it.clone().count());
    for ev in it {
        if let Some(back) = ret.last_mut() {
            match (back, ev) {
                (CE::Pan(v1), CE::Pan(v2)) => {
                    *v1 += *v2;
                }
                (CE::ZoomIn(f1), CE::ZoomIn(f2)) => {
                    *f1 += *f2;
                }
                (CE::ChangeMode(old_mode), CE::ChangeMode(new_mode)) => {
                    *old_mode = new_mode.clone();
                }
                _ => {
                    ret.push(ev.clone());
                }
            }
        } else {
            ret.push(ev.clone());
        }
    }
    ret
}

/// Draw a list of labels
fn labels_ui(labels: impl Iterator<Item = impl Into<egui::WidgetText>>, ui: &mut egui::Ui) {
    for l in labels {
        ui.label(Into::<egui::WidgetText>::into(l));
    }
}
