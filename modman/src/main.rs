use slog::*;
use smithay::backend::egl::EGLGraphicsBackend;
use smithay::backend::graphics::gl::GLGraphicsBackend;
use smithay::backend::input::{InputBackend, InputEvent};
use smithay::reexports::calloop::*;
use std::time::Duration;

mod drawer;

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

    let egl_buffer_reader = graphics_backend
        .bind_wl_display(&display)
        .map_err(|e| {
            crit!(logger, "{}", e);
        })
        .expect("Could not bind wayland display.");



    info!(
        logger,
        "FRAMEBUFFER DIMENSIONS: {:?}",
        graphics_backend.get_framebuffer_dimensions()
    );
    
    // TODO move this to a gl module
    let drawer = {
        let display: smithay::backend::graphics::glium::GliumGraphicsBackend<_> = graphics_backend.into();
        display
    };

    let cloned_log = logger.clone();
    let mut state = ModmanState {};
    let signal = event_loop.get_signal();
    if let Err(e) = event_loop.run(Some(Duration::from_millis(50)), &mut state, move |data| {
        info!(cloned_log, "{:?}", data);
        if let Err(e) = input_backend.dispatch_new_events(|event, _input_config| match event {
            InputEvent::Keyboard { seat, event } => {
                info!(cloned_log, "SEAT: {:?}\nEVENT: {:?}", seat, event);
            }
            _ => {}
        }) {
            error!(cloned_log, "{}", e);
            signal.stop();
        }
    
        // set the clear-color of the window
        drawer.borrow().swap_buffers().unwrap();

    }) {
        error!(logger, "{}", e);
    }

    info!(logger, "Thank you for using modman.");
    Ok(())
}
