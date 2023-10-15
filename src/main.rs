mod camera;
mod input;
mod physics;
mod resources;
mod ui;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::EguiPlugin;
use camera::{handle_view_event, ViewEvent};
use input::{handle_keyboard, handle_mouse};
use physics::{bubble_physics, repel_system, BubblePhysics, GlobalPhysics};
use resources::{InputMode, OccupiedScreenSpace};
use ui::ui_example_system;

use crate::camera::MainCamera;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = create_app();
    app.run();
    Ok(())
}

fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_event::<ViewEvent>()
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<InputMode>()
        .init_resource::<GlobalPhysics>()
        .add_systems(Startup, setup_system)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(Update, handle_mouse)
        .add_systems(Update, handle_keyboard)
        .add_systems(Update, handle_view_event)
        .add_systems(Update, repel_system)
        .add_systems(Update, bubble_physics);
    app
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Circle
    let positions = [
        Vec2::new(-150., 75.),
        Vec2::new(150., 125.),
        Vec2::new(150., -75.),
    ];
    for pos in positions {
        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.))
                    .with_scale(Vec3::new(2., 1., 0.)),
                ..default()
            },
            BubblePhysics {
                vel: Vec2::new(0., 0.),
            },
        ));
    }

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    });
}
