pub mod bubbles;
pub mod camera;
pub mod cursor_control;
pub mod input;
pub mod layers;
pub mod physics;
pub mod ui;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle, utils::HashMap};
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use bubbles::{BubbleId, Bubbles};
use camera::{handle_view_event, ControlEvent};
use cursor_control::CursorControl;
use input::{handle_keyboard, handle_mouse};
use physics::{physics_system, BubblePhysics, GlobalPhysics};
use ui::{ui_system, ControlHistory, OccupiedScreenSpace};

use crate::camera::MainCamera;

pub fn default_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(ShapePlugin)
        .add_event::<ControlEvent>()
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<CursorControl>()
        .init_resource::<GlobalPhysics>()
        .init_resource::<Bubbles>()
        .init_resource::<ControlHistory>()
        .add_systems(Startup, setup_system)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_system)
        .add_systems(Update, (handle_mouse, handle_keyboard))
        .add_systems(Update, handle_view_event)
        .add_systems(Update, physics_system)
        .add_systems(Update, link_physics)
        .add_systems(Update, record_command_history)
        .add_systems(PostUpdate, update_links);
    app
}

fn record_command_history(
    mut commands: EventReader<ControlEvent>,
    mut history: ResMut<ControlHistory>,
) {
    let len = commands.len();
    history.extend_with_len(commands.iter().cloned(), len);
}

fn setup_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Spawn zero marker circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0., 0., 2.)),
        ..default()
    });
}

#[derive(Component)]
pub struct BubbleConnection(BubbleId, BubbleId);

fn update_links(
    mut links: Query<(&mut Path, &BubbleConnection)>,
    q: Query<(&Transform, &GraphBubble)>,
) {
    let tfms_by_id = q
        .iter()
        .map(|(tfm, id)| (id.0, tfm))
        .collect::<HashMap<_, _>>();

    for (mut path, conn) in links.iter_mut() {
        let left = tfms_by_id[&conn.0].translation.truncate();
        let right = tfms_by_id[&conn.1].translation.truncate();
        let line = shapes::Line(left, right);
        *path = GeometryBuilder::build_as(&line);
    }
}

fn link_physics(
    mut q: Query<(&mut BubblePhysics, &Transform, &GraphBubble), With<GraphBubble>>,
    time: Res<Time>,
    bubbles: Res<Bubbles>,
    gphysics: Res<GlobalPhysics>,
) {
    let pos_by_id = q
        .iter()
        .map(|(_, tfm, id)| (id.0, tfm.translation.truncate()))
        .collect::<HashMap<_, _>>();

    for (mut phys, tfm, id) in q.iter_mut() {
        let origin = tfm.translation.truncate();
        let links = bubbles.neighbors(id.0);
        for link in links {
            let id: u32 = link.into();
            // ???: this guards against a crash with an unknown reason
            match pos_by_id.get(&id) {
                Some(tgt) => {
                    // dv = k*x + b
                    let x = *tgt - origin;
                    let k = gphysics.flink;
                    phys.vel += spring_force(k, x) * time.delta_seconds();
                }
                None => {}
            }
        }
    }
}

/// According to Hooke's law
///
/// k * x, where
///
/// - k is stiffness
/// - x is distance
fn spring_force(stiffness: f32, dist: Vec2) -> Vec2 {
    stiffness * dist
}

#[derive(Component, PartialEq, Eq, Hash)]
pub struct GraphBubble(pub BubbleId);
