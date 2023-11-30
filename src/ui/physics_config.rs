use std::ops::RangeInclusive;

use crate::physics::{
    GlobalPhysics, DEFAULT_FCENTER, DEFAULT_FLINK, DEFAULT_FREPEL, DEFAULT_SLOW_MULT,
};
use bevy_egui::egui::{self, CollapsingHeader};
use eframe::emath::Numeric;

pub fn physics_config_ui(ui: &mut egui::Ui, gphysics: &mut GlobalPhysics) {
    CollapsingHeader::new("Physics configurations")
        .default_open(true)
        .show(ui, |ui| {
            // log_slider("Connection length", &mut ?, 0f64..=100.0, ui);
            log_slider(
                "Gravity",
                &mut gphysics.fcenter,
                0.1 * DEFAULT_FCENTER..=10. * DEFAULT_FCENTER,
                ui,
            );
            log_slider(
                "Slow / friction",
                &mut gphysics.slow_mult,
                0.1 * DEFAULT_SLOW_MULT..=10. * DEFAULT_SLOW_MULT,
                ui,
            );
            log_slider(
                "Repel force",
                &mut gphysics.frepel,
                0.1 * DEFAULT_FREPEL..=10. * DEFAULT_FREPEL,
                ui,
            );
            log_slider(
                "Link distance multiplier (k in k*x)",
                &mut gphysics.flink,
                0.1 * DEFAULT_FLINK..=10. * DEFAULT_FLINK,
                ui,
            );
        });
}

fn log_slider<T: Numeric>(text: &str, value: &mut T, range: RangeInclusive<T>, ui: &mut egui::Ui) {
    ui.add(egui::Label::new(text));
    ui.add(egui::Slider::new(value, range).logarithmic(true));
}
