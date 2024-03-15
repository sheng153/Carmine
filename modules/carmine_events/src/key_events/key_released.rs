use crate::event::{Event, EventCategory, EventType};
use crate::key_events::key_event::KeyEvent;

pub struct KeyReleased {
    event: KeyEvent,
    is_handled: bool,
}

impl KeyReleased {
    pub fn new(keycode: u32) -> KeyReleased {
        KeyReleased {
            event: KeyEvent::new(keycode),
            is_handled: false,
        }
    }
    #[inline]
    pub fn keycode(&self) -> u32 {
        self.event.keycode
    }
}

impl Event for KeyReleased {
    #[inline]
    fn get_event_type(&self) -> EventType {
        EventType::KeyReleased
    }
    #[inline]
    fn get_handled(&self) -> bool {
        self.is_handled
    }
    #[inline]
    fn set_handled(&mut self, is_handled: bool) {
        self.is_handled = is_handled;
    }
    #[inline]
    fn get_category_flags() -> u8 {
        1u8 << 1u8 | 1u8 << 2u8
    }
    #[inline]
    fn is_in_category(category: EventCategory) -> bool {
        Self::get_category_flags() & category.value() != 0
    }
    #[inline]
    fn get_categories() -> &'static [EventCategory] {
        &[EventCategory::Keyboard, EventCategory::Input]
    }
}
