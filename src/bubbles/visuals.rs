use bevy::{
    asset::{Assets, Handle},
    ecs::system::Commands,
    hierarchy::BuildChildren,
    math::{Vec2, Vec3},
    prelude::*,
    render::{
        color::Color,
        mesh::{shape, Mesh},
    },
    sprite::{ColorMaterial, MaterialMesh2dBundle, Mesh2dHandle},
    text::{BreakLineOn, Text2dBounds, Text2dBundle},
    transform::components::Transform,
    utils::hashbrown::HashMap,
};
use bevy_prototype_lyon::{
    draw::{Fill, Stroke},
    entity::ShapeBundle,
    geometry::GeometryBuilder,
    shapes,
};

use crate::{bubbles::graph::BubbleId, physics::BubblePhysics, BubbleConnection, GraphBubble};

#[derive(Resource)]
pub struct BubbleBundleBuilder<'c, 'w, 's> {
    commands: &'c mut Commands<'w, 's>,
    bubble_mesh: Mesh2dHandle,
    /// All bubbles are scaled by this factor. Used for determining bubble shape.
    bubble_base_scale: Vec2,
    bubble_material: Handle<ColorMaterial>,
    positions_by_id: HashMap<BubbleId, Vec2>,
}

impl<'c, 'w, 's> BubbleBundleBuilder<'c, 'w, 's> {
    const ELLIPSE_SIZE: Vec2 = Vec2::new(100., 50.);

    pub fn new(
        commands: &'c mut Commands<'w, 's>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let positions_by_id = HashMap::new();
        Self::from_positions_by_id(positions_by_id, commands, meshes, materials)
    }

    pub fn from_positions_by_id(
        positions_by_id: HashMap<BubbleId, Vec2>,
        commands: &'c mut Commands<'w, 's>,
        meshes: &mut Assets<Mesh>,
        materials: &mut Assets<ColorMaterial>,
    ) -> Self {
        let (circle_mesh, bubble_base_scale) = make_scaled_circle(Self::ELLIPSE_SIZE);
        let bubble_mesh = meshes.add(circle_mesh).into();
        let bubble_material = materials.add(ColorMaterial::from(Color::PURPLE));
        Self {
            commands,
            bubble_mesh,
            bubble_base_scale,
            bubble_material,
            positions_by_id,
        }
    }

    pub fn create_bubble(&mut self, id: u32, pos: Vec2) -> BubbleId {
        let bubble = self.create_bubble_bundle(id, pos, self.bubble_base_scale);
        let textbox = create_textbox_bundle(id);

        // Spawn the bubble with a text box as a child
        self.commands.spawn(bubble).with_children(|builder| {
            builder.spawn(Text2dBundle {
                text: textbox,
                text_2d_bounds: Text2dBounds {
                    // Wrap text in the rectangle
                    size: biggest_rectangle_in_ellipse(Self::ELLIPSE_SIZE),
                },
                // Ensure the text is drawn on top
                transform: Transform::from_translation(Vec3::Z).with_scale(Vec3::new(0.5, 1., 1.)),
                ..Default::default()
            });
        });
        id
    }

    fn create_bubble_bundle(
        &mut self,
        id: u32,
        pos: Vec2,
        scale: Vec2,
    ) -> (
        MaterialMesh2dBundle<ColorMaterial>,
        BubblePhysics,
        GraphBubble,
    ) {
        let physics = BubblePhysics::default();
        let bubble = GraphBubble(id);
        let transform = Transform::from_translation(pos.extend(1.)).with_scale(scale.extend(1.));
        self.positions_by_id.insert(id, pos);
        (
            MaterialMesh2dBundle {
                mesh: self.bubble_mesh.clone(),
                material: self.bubble_material.clone(),
                transform,
                ..Default::default()
            },
            physics,
            bubble,
        )
    }

    /// Creates a connection between the `left` bubble and the `right` bubble
    pub fn connect(&mut self, left: BubbleId, right: BubbleId) {
        let left_pos = self.positions_by_id[&left];
        let right_pos = self.positions_by_id[&right];

        // Also create visual entity for the connection
        self.create_line(left_pos, right_pos, left, right);
    }

    pub fn position(&self, bubble: BubbleId) -> Vec2 {
        self.positions_by_id[&bubble]
    }

    fn create_line(&mut self, left_pos: Vec2, right_pos: Vec2, left: u32, right: u32) {
        let bundle = create_line_bundle(left_pos, right_pos, left, right);
        self.commands.spawn(bundle);
    }
}

fn create_line_bundle(
    left_pos: Vec2,
    right_pos: Vec2,
    left_id: u32,
    right_id: u32,
) -> (ShapeBundle, Fill, Stroke, BubbleConnection) {
    let line = shapes::Line(left_pos, right_pos);
    let path = GeometryBuilder::build_as(&line);
    let shape = ShapeBundle { path, ..default() };
    let bundle = (
        shape,
        Fill::color(Color::CYAN),
        Stroke::new(Color::BLACK, 10.0),
        BubbleConnection(left_id, right_id),
    );
    bundle
}

fn create_textbox_bundle(id: u32) -> Text {
    let text_style = TextStyle {
        font_size: 24.,
        color: Color::WHITE,
        ..Default::default()
    };
    let text = Text {
        sections: vec![TextSection::new(
            //"this text wraps in the box\n(AnyCharacter linebreaks)",
            id.to_string(),
            //"Lorem ipsum dolor sit amet consectetur, Lorem ipsum dolor sit amet consectetur, Lorem ipsum dolor sit
            //amet consectetur",
            text_style.clone(),
        )],
        alignment: TextAlignment::Center,
        linebreak_behavior: BreakLineOn::AnyCharacter,
    };
    text
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
