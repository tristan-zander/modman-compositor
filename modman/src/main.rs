use slog::*;
use std::{error::Error, time::Duration};
use smithay::backend::egl::EGLGraphicsBackend;
use smithay::backend::input::{InputBackend, InputEvent};
use smithay::reexports::calloop::*;

#[derive(Debug)]
struct ModmanState {}

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let logger = slog::Logger::root(
        slog_async::Async::default(slog_term::term_full().fuse()).fuse(),
        slog::o!(),
    );

    // initialize event loop
    let mut event_loop: EventLoop<ModmanState> = smithay::reexports::calloop::EventLoop::new()
        .map_err(|e| crit!(logger, "{}", e))
        .expect("");

    // initialize backend
    let (graphics_backend, mut input_backend) = smithay::backend::winit::init(logger.clone())
        .map_err(|e| crit!(logger, "{}", e))
        .expect("Failed to initialize winit backend");

    let display = wayland_server::Display::new();

    let egl_reader = graphics_backend
        .bind_wl_display(&display)
        .map_err(|e| {
            crit!(logger, "{}", e);
        })
        .expect("Could not bind wayland display.");

    let cloned_log = logger.clone();
    let mut state = ModmanState{};
    event_loop.run(Some(Duration::from_secs(1)), &mut state, move |data| {
        info!(cloned_log, "{:?}", data);
        input_backend.dispatch_new_events(|event, input_config| {
            match event {
                InputEvent::Keyboard { seat, event } => { info!(cloned_log, "SEAT: {:?}\nEVENT: {:?}", seat, event); }
                _ => {info!(cloned_log, "Unhandled input event.")}
            }
        }).expect("Failed to finish running event loop.");
    }).map_err(|e| info!(logger, "{}", e)).expect("Error running event loop.");

    info!(logger, "Thank you for using modman.");
    Ok(())
}
