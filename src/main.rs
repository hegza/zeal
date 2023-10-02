struct MyApp {
    name: String,
    age: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            name: "Arthur".to_owned(),
            age: 42,
        }
    }
}

/*
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &bevy::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                // TODO: the bevy_egui view with the bubbles
                //ui.
                // TODO: configurations bar
                ui.vertical(|ui| ui.collapsing("Test", ui.vertical(|ui| ui.label("Test2"))))
            })
            /*
            ui.heading("My egui Application");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut self.name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
            if ui.button("Click each year").clicked() {
                self.age += 1;
            }
            ui.label(format!("Hello '{}', age {}", self.name, self.age));
            */
        });
    }
}
*/

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};

fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}

fn main() -> Result<(), eframe::Error> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .run();

    /*
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640.0, 480.0)),
        run_and_return: false,
        ..Default::default()
    };
    eframe::run_native(
        env!("CARGO_PKG_NAME"),
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )*/
    Ok(())
}

