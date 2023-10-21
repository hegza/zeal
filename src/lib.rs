pub mod bubble_graph;
pub mod camera;
pub mod cursor_control;
pub mod input;
pub mod layers;
pub mod physics;
pub mod ui;

use bevy::{
    prelude::*,
    sprite::MaterialMesh2dBundle,
    text::{BreakLineOn, Text2dBounds},
    utils::HashMap,
};
use bevy_egui::EguiPlugin;
use bevy_prototype_lyon::prelude::*;
use bubble_graph::{BubbleGraph, BubbleId};
use camera::{handle_view_event, ControlEvent};
use cursor_control::CursorControl;
use input::{handle_keyboard, handle_mouse};
use physics::{bubble_physics, repel_system, BubblePhysics, GlobalPhysics};
use ui::{ui_example_system, ControlHistory, OccupiedScreenSpace};

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
        .init_resource::<BubbleGraph>()
        .init_resource::<ControlHistory>()
        .add_systems(Startup, setup_system)
        // Systems that create Egui widgets should be run during the `CoreSet::Update` set,
        // or after the `EguiSet::BeginFrame` system (which belongs to the `CoreSet::PreUpdate` set).
        .add_systems(Update, ui_example_system)
        .add_systems(Update, (handle_mouse, handle_keyboard))
        .add_systems(Update, handle_view_event)
        .add_systems(Update, repel_system)
        .add_systems(Update, bubble_physics)
        .add_systems(Update, link_physics)
        .add_systems(Update, record_command_history)
        .add_systems(PostUpdate, update_links);
    app
}

fn record_command_history(
    mut commands: EventReader<ControlEvent>,
    mut history: ResMut<ControlHistory>,
) {
    // HACK: should not need to allocate a vec here
    let v = commands.iter().cloned().collect::<Vec<_>>();
    history.extend(v.into_iter());
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

pub struct BubbleGraphBuilder<'c, 'w, 's, 'g, 'me, 'ma, 'asset> {
    commands: &'c mut Commands<'w, 's>,
    graph: &'g mut BubbleGraph,
    meshes: &'me mut Assets<Mesh>,
    materials: &'ma mut Assets<ColorMaterial>,
    _asset_server: &'asset AssetServer,
    positions_by_id: HashMap<BubbleId, Vec2>,
}

/// Returns a circle mesh and a scaling vector
fn make_scaled_circle(size: Vec2) -> (Mesh, Vec2) {
    let (radius, scale) = if size.x < size.y {
        (size.x, Vec2::new(1.0, size.y / size.x))
    } else {
        (size.y, Vec2::new(size.x / size.y, 1.0))
    };
    (shape::Circle::new(radius).into(), scale)
}

fn biggest_rectangle_in_ellipse(ellipse_size: Vec2) -> Vec2 {
    Vec2::new(2f32.sqrt() * ellipse_size.x, 2f32.sqrt() * ellipse_size.y)
}

impl<'c, 'w, 's, 'g, 'me, 'ma, 'asset> BubbleGraphBuilder<'c, 'w, 's, 'g, 'me, 'ma, 'asset> {
    pub fn new(
        commands: &'c mut Commands<'w, 's>,
        graph: &'g mut BubbleGraph,
        meshes: &'me mut Assets<Mesh>,
        materials: &'ma mut Assets<ColorMaterial>,
        asset_server: &'asset AssetServer,
    ) -> Self {
        let positions_by_id = HashMap::new();
        Self::from_positions_by_id(
            positions_by_id,
            commands,
            graph,
            meshes,
            materials,
            asset_server,
        )
    }

    pub fn from_positions_by_id(
        positions_by_id: HashMap<BubbleId, Vec2>,
        commands: &'c mut Commands<'w, 's>,
        graph: &'g mut BubbleGraph,
        meshes: &'me mut Assets<Mesh>,
        materials: &'ma mut Assets<ColorMaterial>,
        _asset_server: &'asset AssetServer,
    ) -> Self {
        Self {
            commands,
            graph,
            meshes,
            materials,
            positions_by_id,
            _asset_server,
        }
    }

    pub fn create_bubble(&mut self, pos: Vec2) -> BubbleId {
        let physics = BubblePhysics::default();
        let id = self.graph.insert();
        let bubble = GraphBubble(id);
        let ellipse_size = Vec2::new(100., 50.);
        let (circle_mesh, scale) = make_scaled_circle(ellipse_size);
        let mesh = self.meshes.add(circle_mesh).into();
        let material = self.materials.add(ColorMaterial::from(Color::PURPLE));
        let transform = Transform::from_translation(pos.extend(1.)).with_scale(scale.extend(1.));
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
        let text_style = TextStyle {
            font_size: 24.,
            color: Color::WHITE,
            ..Default::default()
        };

        // Spawn the bubble with a text box as a child
        let text = Text {
            sections: vec![TextSection::new(
                //"this text wraps in the box\n(AnyCharacter linebreaks)",
                id.to_string(),
                //"Lorem ipsum dolor sit amet consectetur, Lorem ipsum dolor sit amet consectetur, Lorem ipsum dolor sit amet consectetur",
                text_style.clone(),
            )],
            alignment: TextAlignment::Center,
            linebreak_behavior: BreakLineOn::AnyCharacter,
        };
        self.commands.spawn(bundle).with_children(|builder| {
            builder.spawn(Text2dBundle {
                text,
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: biggest_rectangle_in_ellipse(ellipse_size),
                },
                // Ensure the text is drawn on top
                transform: Transform::from_translation(Vec3::Z).with_scale(Vec3::new(0.5, 1., 1.)),
                ..default()
            });
        });
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
            // ???: this guards against a crash with an unknown reason
            match pos_by_id.get(&id) {
                Some(tgt) => {
                    // dv = k*x + b
                    let x = *tgt - origin;
                    let k = gphysics.flink;
                    let b = gphysics.link_base;
                    phys.vel += (k * x + b) * time.delta_seconds();
                }
                None => {}
            }
        }
    }
}

#[derive(Component, PartialEq, Eq, Hash)]
pub struct GraphBubble(pub BubbleId);
