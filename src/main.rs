use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

static mut CONNECTION_LENGTH: f64 = 20.0;

fn ui_example_system(
    mut contexts: EguiContexts,
    mut occupied_screen_space: ResMut<OccupiedScreenSpace>,
) {
    let ctx = contexts.ctx_mut();

    occupied_screen_space.left = egui::SidePanel::left("left_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Left resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.right = egui::SidePanel::right("right_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Right resizeable panel");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.collapsing("Physics configurations", |ui| {
                        ui.add(egui::Slider::new(
                            &mut unsafe { CONNECTION_LENGTH },
                            0f64..=100.0,
                        ));
                        ui.vertical(|ui| ui.label("Test2"))
                    })
                })
            });
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();
    occupied_screen_space.top = egui::TopBottomPanel::top("top_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Top resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    occupied_screen_space.bottom = egui::TopBottomPanel::bottom("bottom_panel")
        .resizable(true)
        .show(ctx, |ui| {
            ui.label("Bottom resizeable panel");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .height();
    // TODO: the bevy_egui view with the bubbles
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .init_resource::<OccupiedScreenSpace>()
        .add_systems(Startup, setup_system)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(Update, update_cam)
        .run();
    Ok(())
}

#[derive(Default, Resource)]
struct OccupiedScreenSpace {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

#[derive(Component)]
struct MainCamera;

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(50.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::PURPLE)),
        transform: Transform::from_translation(Vec3::new(-150., 0., 0.))
            .with_scale(Vec3::new(2., 1., 0.)),
        ..default()
    });

    // Rectangle
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.25, 0.25, 0.75),
            custom_size: Some(Vec2::new(50.0, 100.0)),
            ..default()
        },
        transform: Transform::from_translation(Vec3::new(-50., 0., 0.)),
        ..default()
    });

    // Quad
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes
            .add(shape::Quad::new(Vec2::new(50., 100.)).into())
            .into(),
        material: materials.add(ColorMaterial::from(Color::LIME_GREEN)),
        transform: Transform::from_translation(Vec3::new(50., 0., 0.)),
        ..default()
    });

    // Hexagon
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
        material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
        transform: Transform::from_translation(Vec3::new(150., 0., 0.)),
        ..default()
    });
}

fn update_cam(time: Res<Time>, mut q: Query<&mut OrthographicProjection, With<MainCamera>>) {
    let mut projection = q.single_mut();
    projection.viewport_origin += Vec2::new(0.01, 0.) * time.delta_seconds();
}
