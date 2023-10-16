use bevy::utils::HashMap;
use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use rand::Rng;
use zeal::bubble_graph::BubbleId;
use zeal::camera::{handle_view_event, ViewEvent};
use zeal::input::{handle_keyboard, handle_mouse};
use zeal::physics::{bubble_physics, repel_system, BubblePhysics, GlobalPhysics};
use zeal::resources::{InputMode, OccupiedScreenSpace};
use zeal::ui::ui_example_system;
use zeal::{bubble_graph::BubbleGraph, camera::MainCamera};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = create_app();
    app.run();
    Ok(())
}

fn create_app() -> App {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugins(EguiPlugin)
        .add_plugins(ShapePlugin)
        .add_event::<ViewEvent>()
        .init_resource::<OccupiedScreenSpace>()
        .init_resource::<InputMode>()
        .init_resource::<GlobalPhysics>()
        .init_resource::<BubbleGraph>()
        .init_resource::<Countdown>()
        .add_systems(Startup, setup_system)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(Update, handle_mouse)
        .add_systems(Update, handle_keyboard)
        .add_systems(Update, handle_view_event)
        .add_systems(Update, link_physics)
        .add_systems(Update, repel_system)
        .add_systems(Update, bubble_physics)
        .add_systems(Update, countdown)
        .add_systems(Update, update_links);
    app
}

fn create_bubble(
    pos: Vec2,
    graph: &mut BubbleGraph,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
) -> (impl Bundle, BubbleId) {
    let physics = BubblePhysics {
        vel: Vec2::new(0., 0.),
    };
    let id = graph.add_bubble();
    let bubble = GraphBubble(id);
    (
        (
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(50.).into()).into(),
                material: materials.add(ColorMaterial::from(Color::PURPLE)),
                transform: Transform::from_translation(Vec3::new(pos.x, pos.y, 0.))
                    .with_scale(Vec3::new(2., 1., 0.)),
                ..default()
            },
            physics,
            bubble,
        ),
        id,
    )
}

fn create_connection(
    bubble: BubbleId,
    left: Vec2,
    right: Vec2,
    graph: &mut BubbleGraph,
) -> impl Bundle {
    // TODO: system to update all connections with their bubble parents

    let main = unsafe { MAIN_BUBBLE_ID.as_ref().unwrap() };

    graph.connect(bubble, *main).unwrap();

    // Also create visual entity for the connection
    let line = shapes::Line(left, right);
    let path = GeometryBuilder::build_as(&line);
    let shape = ShapeBundle { path, ..default() };
    (
        shape,
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 10.0),
        BubbleConnection(bubble, *main),
    )
}

#[derive(Component)]
struct MainBubble;

#[derive(Component, PartialEq, Eq, Hash)]
pub struct GraphBubble(BubbleId);

#[derive(Resource)]
pub struct Countdown {
    pub timer: Timer,
}

impl Default for Countdown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.5f32, TimerMode::Repeating),
        }
    }
}

static mut MAIN_BUBBLE_ID: Option<BubbleId> = None;

#[derive(Component)]
pub struct BubbleConnection(BubbleId, BubbleId);

fn setup_system(
    mut graph: ResMut<BubbleGraph>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Make a bubble
    let physics = BubblePhysics {
        vel: Vec2::new(0., 0.),
    };
    let id = graph.add_bubble();
    unsafe {
        let _ = MAIN_BUBBLE_ID.insert(id);
    };
    let bubble = GraphBubble(id);
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(50.).into()).into(),
            material: materials.add(ColorMaterial::from(Color::PURPLE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.))
                .with_scale(Vec3::new(2., 1., 0.)),
            ..default()
        },
        physics,
        MainBubble,
        bubble,
    ));

    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    });
}

fn countdown(
    time: Res<Time>,
    mut countdown: ResMut<Countdown>,
    mut graph: ResMut<BubbleGraph>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    if countdown.timer.tick(time.delta()).just_finished() {
        // TODO: use better numbers
        let left = Vec2::new(0., 0.);
        let mut rng = rand::thread_rng();
        let right = Vec2::new(0. + rng.gen::<f32>() * 20., -150.);
        let (bubble, id) = create_bubble(right, &mut graph, &mut meshes, &mut materials);
        commands.spawn(bubble);
        commands.spawn(create_connection(id, left, right, &mut graph));
    }
}

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
    graph: Res<BubbleGraph>,
    gphysics: Res<GlobalPhysics>,
) {
    let pos_by_id = q
        .iter()
        .map(|(_, tfm, id)| (id.0, tfm.translation.truncate()))
        .collect::<HashMap<_, _>>();

    for (mut phys, tfm, id) in q.iter_mut() {
        let origin = tfm.translation.truncate();
        let links = graph.neighbors(id.0);
        for link in links {
            let id: u32 = link.into();
            let tgt = pos_by_id[&id];

            // dv = k*x + b
            let x = tgt - origin;
            let k = gphysics.flink;
            let b = gphysics.link_base;
            phys.vel += (k * x + b) * time.delta_seconds();
        }
    }
}
