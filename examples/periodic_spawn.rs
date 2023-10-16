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

struct BubbleGraphBuilder<'c, 'me, 'ma, 'g, 'w, 's> {
    commands: &'c mut Commands<'w, 's>,
    graph: &'g mut BubbleGraph,
    meshes: &'me mut Assets<Mesh>,
    materials: &'ma mut Assets<ColorMaterial>,
    positions_by_id: HashMap<BubbleId, Vec2>,
}

impl<'c, 'me, 'ma, 'g, 'w, 's> BubbleGraphBuilder<'c, 'me, 'ma, 'g, 'w, 's> {
    pub fn new(
        commands: &'c mut Commands<'w, 's>,
        graph: &'g mut BubbleGraph,
        meshes: &'me mut Assets<Mesh>,
        materials: &'ma mut Assets<ColorMaterial>,
    ) -> Self {
        let positions_by_id = HashMap::new();
        Self::from_positions_by_id(positions_by_id, commands, graph, meshes, materials)
    }

    pub fn from_positions_by_id(
        positions_by_id: HashMap<BubbleId, Vec2>,
        commands: &'c mut Commands<'w, 's>,
        graph: &'g mut BubbleGraph,
        meshes: &'me mut Assets<Mesh>,
        materials: &'ma mut Assets<ColorMaterial>,
    ) -> Self {
        Self {
            commands,
            graph,
            meshes,
            materials,
            positions_by_id,
        }
    }

    pub fn create_bubble(&mut self, pos: Vec2) -> BubbleId {
        let physics = BubblePhysics {
            vel: Vec2::new(0., 0.),
        };
        let id = self.graph.insert();
        let bubble = GraphBubble(id);
        let mesh = self.meshes.add(shape::Circle::new(50.).into()).into();
        let material = self.materials.add(ColorMaterial::from(Color::PURPLE));
        let transform =
            Transform::from_translation(pos.extend(0.)).with_scale(Vec3::new(2., 1., 0.));
        self.positions_by_id.insert(id, pos);
        let bundle = (
            MaterialMesh2dBundle {
                mesh,
                material,
                transform,
                ..default()
            },
            physics,
            bubble,
        );
        self.commands.spawn(bundle);
        id
    }

    pub fn connect(&mut self, left: BubbleId, right: BubbleId) {
        assert!(self.graph.contains_node(left));
        assert!(self.graph.contains_node(right));

        self.graph.add_edge(left, right).unwrap();

        let left_pos = self.positions_by_id[&left];
        let right_pos = self.positions_by_id[&right];

        // Also create visual entity for the connection
        let line = shapes::Line(left_pos, right_pos);
        let path = GeometryBuilder::build_as(&line);
        let shape = ShapeBundle { path, ..default() };
        let bundle = (
            shape,
            Fill::color(Color::CYAN),
            Stroke::new(Color::BLACK, 10.0),
            BubbleConnection(left, right),
        );
        self.commands.spawn(bundle);
    }
}

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

#[derive(Component)]
pub struct BubbleConnection(BubbleId, BubbleId);

static mut MAIN_BUBBLE_ID: Option<BubbleId> = None;

fn setup_system(
    mut graph: ResMut<BubbleGraph>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Make a bubble
    let mut graph = BubbleGraphBuilder::new(&mut commands, &mut graph, &mut meshes, &mut materials);

    let id = graph.create_bubble(Vec2::ZERO);
    unsafe {
        let _ = MAIN_BUBBLE_ID.insert(id);
    };

    // Spawn zero marker circle
    commands.spawn(MaterialMesh2dBundle {
        mesh: meshes.add(shape::Circle::new(5.).into()).into(),
        material: materials.add(ColorMaterial::from(Color::RED)),
        transform: Transform::from_translation(Vec2::ZERO.extend(1.)),
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
    q: Query<(&GraphBubble, &Transform)>,
) {
    let pos_by_id = q
        .iter()
        .map(|(id, tfm)| (id.0, tfm.translation.truncate()))
        .collect::<HashMap<BubbleId, Vec2>>();
    let mut builder = BubbleGraphBuilder::from_positions_by_id(
        pos_by_id,
        &mut commands,
        &mut graph,
        &mut meshes,
        &mut materials,
    );

    if countdown.timer.tick(time.delta()).just_finished() {
        let mut rng = rand::thread_rng();
        let pos = Vec2::new(0. + rng.gen::<f32>() * 20., -150.);
        let id = builder.create_bubble(pos);
        if let Some(prime) = unsafe { MAIN_BUBBLE_ID } {
            builder.connect(prime, id);
        }
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
