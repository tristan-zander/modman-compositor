use slog::info;
use slog::Logger;
use smithay::reexports::wayland_server::Display;

pub struct UdevBackend {}

pub fn run_udev(display: &mut Display, log: Logger) -> Result<(), Box<dyn std::error::Error>> {
    let env = display
        .add_socket_auto()
        .expect("Could not create wayland socket.");

    

    Ok(())
}
