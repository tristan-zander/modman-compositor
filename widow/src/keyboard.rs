use smithay::backend::input::KeyboardKeyEvent as KeyboardEvent;
use smithay::wayland::seat::keysyms as xkb;

#[non_exhaustive]
pub enum WMAction {
    CloseWindow,
    None,
}

pub fn handle_keyboard_event<T: KeyboardEvent>(event: T) -> WMAction {
    let keycode = event.key_code();

    use WMAction::*;
    let ret = match keycode {
        xkb::KEY_BackSpace => CloseWindow,
        _ => None,
    };

    ret
}
