use slog::{crit, info, error, Logger};
use smithay::reexports::wayland_server::Display;

use glium::Surface;
use smithay::backend::egl::EGLGraphicsBackend;
use smithay::backend::graphics::gl::GLGraphicsBackend;
use smithay::backend::input::{InputBackend, InputEvent};
use smithay::reexports::calloop::*;
use std::time::Duration;

use crate::state::WMState;

pub fn run_winit(display: &mut Display, logger: Logger) -> Result<(), Box<dyn std::error::Error>> {
    // initialize backend
    let (graphics_backend, mut input_backend) = smithay::backend::winit::init(logger.clone())
        .map_err(|e| crit!(logger, "{}", e))
        .expect("Failed to initialize winit backend");
    
    let egl_buffer_reader = graphics_backend
        .bind_wl_display(&display)
        .map_err(|e| {
            crit!(logger, "{}", e);
        })
        .expect("Could not bind wayland display.");

    // initialize event loop
    let mut event_loop: EventLoop<WMState> = smithay::reexports::calloop::EventLoop::new()
        .map_err(|e| crit!(logger, "{}", e))
        .expect("");

    info!(
        logger,
        "FRAMEBUFFER DIMENSIONS: {:?}",
        graphics_backend.get_framebuffer_dimensions()
    );

    // TODO move this to a gl module
    let drawer = {
        let display: smithay::backend::graphics::glium::GliumGraphicsBackend<_> =
            graphics_backend.into();
        display
    };

    let cloned_log = logger.clone();
    let mut state = WMState::new(logger.clone());
    let signal = event_loop.get_signal();
    if let Err(e) = event_loop.run(Some(Duration::from_millis(50)), &mut state, move |data| {
        // info!(cloned_log, "{:?}", data);
        if let Err(e) = input_backend.dispatch_new_events(|event, _input_config| match event {
            InputEvent::Keyboard { seat, event } => {
                info!(cloned_log, "SEAT: {:?}\nEVENT: {:?}", seat, event);
                let wm_action = crate::keyboard::handle_keyboard_event(event);

                match wm_action {
                    CloseWindow => {
                        signal.stop();
                    }
                    _ => {}
                }
            }
            _ => {}
        }) {
            error!(cloned_log, "{}", e);
            signal.stop();
        }

        let mut frame = drawer.draw();

        // set the clear-color of the window
        frame.clear_color(0.3, 0.3, 0.3, 0.8);

        frame.finish().unwrap();
    }) {
        return Err(format!("{}", e).into());
    }

    Ok(())
}
