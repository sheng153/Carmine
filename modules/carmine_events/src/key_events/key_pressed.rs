use crate::event::{Event, EventCategory, EventType};
use crate::key_events::key_event::KeyEvent;

pub struct KeyPressed {
    pub repeat_count: bool,
    event: KeyEvent,
    is_handled: bool,
}

impl KeyPressed {
    pub fn new(keycode: u32, repeat_count: bool) -> KeyPressed {
        KeyPressed {
            event: KeyEvent::new(keycode),
            repeat_count,
            is_handled: false,
        }
    }
    #[inline]
    pub fn keycode(&self) -> u32 {
        self.event.keycode
    }
}

impl Event for KeyPressed {
    #[inline]
    fn get_event_type(&self) -> EventType {
        EventType::KeyPressed
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
