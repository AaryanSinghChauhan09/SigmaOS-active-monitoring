//! SigmaOS-active-monitoring - SigmaOS Component

use anyhow::Result;

fn main() -> Result<()> {
    env_logger::init();
    log::info!("Starting SigmaOS-active-monitoring");
    println!("SigmaOS SigmaOS-active-monitoring v0.1.0");
    Ok(())
}
