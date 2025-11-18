mod app;
mod audio;
mod ui;
mod sequencer;
mod config;

use anyhow::Result;

fn main() -> Result<()> {
    // Initialize logger-friendly panic hook
    std::panic::set_hook(Box::new(|info| {
        eprintln!("panic: {}", info);
    }));

    // Run the app (blocking until exit)
    let mut app = app::App::new()?;
    app.run()?;
    Ok(())
}
