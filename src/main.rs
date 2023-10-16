mod bubble_graph;
mod camera;
mod input;
mod layers;
mod physics;
mod resources;
mod ui;

use zeal::default_app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = default_app();
    app.run();
    Ok(())
}
