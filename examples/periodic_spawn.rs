use bevy::prelude::*;
use bevy::utils::HashMap;
use zeal::bubbles::{BubbleBundleBuilder, BubbleId, Bubbles};
use zeal::physics::GlobalPhysics;
use zeal::{default_app, GraphBubble};

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
    mut bubbles: ResMut<Bubbles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Make a bubble
    let mut graph = BubbleBundleBuilder::new(&mut commands, &mut meshes, &mut materials);
    let id = bubbles.spawn_orphan(Vec2::ZERO, &mut graph);

    unsafe {
        let _ = MAIN_BUBBLE_ID.insert(id);
    };
}

fn countdown(
    time: Res<Time>,
    mut countdown: ResMut<Countdown>,
    mut bubbles: ResMut<Bubbles>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut physics: ResMut<GlobalPhysics>,
    q: Query<(&GraphBubble, &Transform)>,
) {
    let pos_by_id = q
        .iter()
        .map(|(id, tfm)| (id.0, tfm.translation.truncate()))
        .collect::<HashMap<BubbleId, Vec2>>();

    let mut builder = BubbleBundleBuilder::from_positions_by_id(
        pos_by_id,
        &mut commands,
        &mut meshes,
        &mut materials,
    );

    if countdown.timer.tick(time.delta()).just_finished() {
        if let Some(prime) = unsafe { MAIN_BUBBLE_ID } {
            // Unwrap is safe because we know that the prime bubble was spawned in init
            bubbles
                .spawn_child(prime, &mut builder, &mut physics)
                .unwrap();
        }
    }
}
