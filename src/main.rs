use termdino::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = App::new()?;
    app.run()?;
    Ok(())
}
