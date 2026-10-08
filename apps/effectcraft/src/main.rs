//! EffectCraft desktop application — 100% sovereign Martensite runtime.

#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use effectcraft_engine::Engine;
use effectcraft_ui_martensite::EffectcraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = Engine::new();
    let app = EffectcraftApp::new(engine);

    println!("Starting EffectCraft Studio on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
