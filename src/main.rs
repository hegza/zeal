mod components;
mod events;
mod resources;
mod ui;

use bevy::{input::mouse::MouseMotion, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::EguiPlugin;
use components::{BubblePhysics, MainCamera};
use events::ViewMoveEvent;
use resources::{GlobalPhysics, InputMode, OccupiedScreenSpace};
use ui::ui_example_system;

static mut CONNECTION_LENGTH: f64 = 20.0;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_event::<ViewMoveEvent>()
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
        .add_systems(Update, bubble_physics)
        .add_systems(Update, update_model)
        .run();
    Ok(())
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut gphysics: ResMut<GlobalPhysics>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Circle
    let pos = Vec2::new(-150., 0.);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.))
                .with_scale(Vec3::new(2., 1., 0.)),
            ..default()
        },
        BubblePhysics {
            pos,
            vel: Vec2::new(0., 0.),
        },
    ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    });

    // Configure physics
    const DEFAULT_FCENTER: f32 = 1.;
    gphysics.fcenter = DEFAULT_FCENTER;
}

/// # Documentation
///
/// Input handling: https://bevy-cheatbook.github.io/builtins.html#input-handling-resources
/// Input event list: https://bevy-cheatbook.github.io/builtins.html#input-events
fn handle_mouse(
    btn_state: Res<Input<MouseButton>>,
    mut mouse_motion: EventReader<MouseMotion>,
    mut view_moves: EventWriter<ViewMoveEvent>,
) {
    let lmb_pressed = btn_state.pressed(MouseButton::Left);
    if lmb_pressed {
        for motion in mouse_motion.iter() {
            view_moves.send(ViewMoveEvent::new(-motion.delta.x, -motion.delta.y))
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
fn handle_keyboard(
    time: Res<Time>,
    skeyboard: Res<Input<KeyCode>>,
    mut view_moves: EventWriter<ViewMoveEvent>,
) {
    if let Some(v) = arrow_keys_to_vec(&skeyboard) {
        const KB_MOVE_PX_PER_SEC: f32 = 500.;
        view_moves.send(ViewMoveEvent::from(
            v * KB_MOVE_PX_PER_SEC * time.delta_seconds(),
        ));
    }
}

fn handle_view_event(
    mut view_moves: EventReader<ViewMoveEvent>,
    mut q: Query<&mut OrthographicProjection, With<MainCamera>>,
) {
    for motion in view_moves.iter() {
        let mut projection = q.single_mut();
        let a = &projection.area;
        let mov = Vec2::new(-motion.x() / a.width(), motion.y() / a.height());
        projection.viewport_origin += mov;
    }
}

fn bubble_physics(time: Res<Time>, gphysics: Res<GlobalPhysics>, mut q: Query<&mut BubblePhysics>) {
    for mut bubble in q.iter_mut() {
        // Apply centering force
        let fcenter = -bubble.pos * gphysics.fcenter;
        bubble.vel += fcenter * time.delta_seconds();

        // Apply velocity
        let delta = bubble.vel * time.delta_seconds();
        bubble.pos += delta;
    }
}

fn update_model(mut q: Query<(&mut Transform, &BubblePhysics)>) {
    for (mut t, p) in q.iter_mut() {
        t.translation = Vec3::new(p.pos.x, p.pos.y, 0.);
    }
}
