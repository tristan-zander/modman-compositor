use slog::*;

use crate::udev::run_udev;
use crate::winit::run_winit;

mod drawer;
mod keyboard;
mod state;
mod udev;
mod winit;

#[derive(Debug)]
struct ModmanState {}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let logger = slog::Logger::root(
        slog_async::Async::default(slog_term::term_full().fuse()).fuse(),
        slog::o!(),
    );

    let mut display = wayland_server::Display::new();

    // TODO: better argument handling with a framework.
    if let Some(arg) = std::env::args().nth(1) {
        match arg.as_str() {
            "--udev" => run_udev(&mut display, logger.clone())?,
            "--winit" => run_winit(&mut display, logger.clone())?,
            _ => {
                return Err("The selected backend is not available in your build of widow.".into());
            }
        }
    } else {
        return Err("No backend was selected.".into());
    }

    info!(logger, "Thank you for using modman.");
    Ok(())
}
