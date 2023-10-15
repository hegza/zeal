use crate::{
    physics::{GlobalPhysics, DEFAULT_FCENTER, DEFAULT_FREPEL, DEFAULT_SLOW_MULT},
    resources::{InputMode, OccupiedScreenSpace},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, CollapsingHeader},
    EguiContexts,
};

/// Capitalizes the first character in s.
fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
    input_mode: Res<InputMode>,
    mut gphysics: ResMut<GlobalPhysics>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = left_panel(ctx, &input_mode);
    occupied_screen_space.right = right_panel(ctx, &mut gphysics);
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

fn right_panel(ctx: &mut egui::Context, gphysics: &mut GlobalPhysics) -> f32 {
    egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    CollapsingHeader::new("Physics configurations").show(ui, |ui| {
                        /*ui.add(egui::Label::new("Connection length"));
                        ui.add(egui::Slider::new(
                            &mut unsafe { crate::CONNECTION_LENGTH },
                            0f64..=100.0,
                        ));
                        */
                        ui.add(egui::Label::new("Repel force"));
                        ui.add(
                            egui::Slider::new(
                                &mut gphysics.frepel,
                                0.1 * DEFAULT_FREPEL..=10. * DEFAULT_FREPEL,
                            )
                            .logarithmic(true),
                        );
                        ui.add(egui::Label::new("Gravity"));
                        ui.add(
                            egui::Slider::new(
                                &mut gphysics.fcenter,
                                0.1 * DEFAULT_FCENTER..=10. * DEFAULT_FCENTER,
                            )
                            .logarithmic(true),
                        );
                        ui.add(egui::Label::new("Slow / friction"));
                        ui.add(
                            egui::Slider::new(
                                &mut gphysics.slow_mult,
                                0.1 * DEFAULT_SLOW_MULT..=10. * DEFAULT_SLOW_MULT,
                            )
                            .logarithmic(true),
                        );
                    });
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
