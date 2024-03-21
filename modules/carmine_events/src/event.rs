#[derive(Clone, Debug)]
pub enum EventCategory {
    None,
    Application,
    Input,
    Keyboard,
    Mouse,
    MouseButton,
}

pub const fn generate_mask(categories: &[EventCategory]) -> u8 {
    let mut ret = 0u8;
    let mut index = 0;
    loop {
        if categories.len() <= index {
            break;
        }
        ret |= categories[index].value();
        index += 1;
    }
    ret
}

impl EventCategory {
    pub const fn value(&self) -> u8 {
        match self {
            EventCategory::None => 0,
            EventCategory::Application => 1 << 0,
            EventCategory::Input => 1 << 1,
            EventCategory::Keyboard => 1 << 2,
            EventCategory::Mouse => 1 << 3,
            EventCategory::MouseButton => 1 << 4,
        }
    }
}

#[derive(Clone, Debug)]
pub enum EventType {
    None,
    WindowClose,
    WindowResize,
    WindowFocus,
    WindowLostFocus,
    WindowMoved,
    AppTick,
    AppUpdate,
    AppRender,
    KeyPressed,
    KeyReleased,
    KeyTyped,
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

pub trait Event {
    fn get_event_type(&self) -> EventType;
    fn get_handled(&self) -> bool;
    fn set_handled(&mut self, handled: bool);
    fn get_category_flags() -> u8;
    fn is_in_category(category: EventCategory) -> bool;
    fn get_categories() -> &'static [EventCategory];
}
