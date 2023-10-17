use bevy::prelude::*;
use bevy::utils::HashMap;
use rand::Rng;
use zeal::bubble_graph::{BubbleGraph, BubbleId};
use zeal::{default_app, BubbleGraphBuilder, GraphBubble};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = create_app();
    app.run();
    Ok(())
}

fn create_app() -> App {
    let mut app = default_app();
    app.init_resource::<Countdown>()
        .add_systems(Startup, extra_setup)
        .add_systems(Update, countdown);
    app
}

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

fn extra_setup(
    mut graph: ResMut<BubbleGraph>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut asset_server: Res<AssetServer>,
) {
    // Make a bubble
    let mut graph = BubbleGraphBuilder::new(
        &mut commands,
        &mut graph,
        &mut meshes,
        &mut materials,
        &mut asset_server,
    );

    let id = graph.create_bubble(Vec2::ZERO);
    unsafe {
        let _ = MAIN_BUBBLE_ID.insert(id);
    };
}

fn countdown(
    time: Res<Time>,
    mut countdown: ResMut<Countdown>,
    mut graph: ResMut<BubbleGraph>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    q: Query<(&GraphBubble, &Transform)>,
    asset_server: Res<AssetServer>,
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
        &asset_server,
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
