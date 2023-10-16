use zeal::default_app;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = default_app();
    app.run();
    Ok(())
}
