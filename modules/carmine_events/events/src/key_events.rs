use carmine_events::{EventCategory, EventType};
use events_codegen::event;

pub(crate) static KEY_EVENT_CATEGORIES: &[EventCategory] =
    &[EventCategory::Keyboard, EventCategory::Input];

#[derive(Clone)]
pub(crate) struct KeyEvent {
    pub(crate) keycode: u32,
}

impl KeyEvent {
    pub(crate) fn new(keycode: u32) -> Self {
        Self { keycode }
    }
}

#[event(&KEY_EVENT_CATEGORIES)]
pub struct KeyPressed {
    pub repeat_count: bool,
    event: KeyEvent,
    handled: bool,
}

impl KeyPressed {
    pub fn new(keycode: u32, repeat_count: bool) -> KeyPressed {
        KeyPressed {
            event: KeyEvent::new(keycode),
            repeat_count,
            handled: false,
        }
    }
    #[inline]
    pub fn keycode(&self) -> u32 {
        self.event.keycode
    }
}

#[event(&KEY_EVENT_CATEGORIES)]
pub struct KeyReleased {
    event: KeyEvent,
    handled: bool,
}

impl KeyReleased {
    pub fn new(keycode: u32) -> KeyReleased {
        KeyReleased {
            event: KeyEvent::new(keycode),
            handled: false,
        }
    }
    #[inline]
    pub fn keycode(&self) -> u32 {
        self.event.keycode
    }
}

#[event(&KEY_EVENT_CATEGORIES)]
pub struct KeyTyped {
    event: KeyEvent,
    handled: bool,
}

impl KeyTyped {
    pub fn new(keycode: u32) -> KeyTyped {
        KeyTyped {
            event: KeyEvent::new(keycode),
            handled: false,
        }
    }
    #[inline]
    pub fn keycode(&self) -> u32 {
        self.event.keycode
    }
}
