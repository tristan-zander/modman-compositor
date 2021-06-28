use slog::Logger;
use smithay::wayland::seat::{KeyboardHandle, PointerHandle};

pub struct WMState {
    //keyboard: KeyboardHandle,
    //mouse: PointerHandle,
    log: Logger,
}

impl WMState {
    pub fn new(log: Logger) -> Self {
        WMState { log: log.clone() }
    }
}
