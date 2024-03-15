pub enum EventCategory {
    None,
    Application,
    Input,
    Keyboard,
    Mouse,
    MouseButton,
}

impl EventCategory {
    pub fn value(&self) -> u8 {
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
    MouseButtonPressed,
    MouseButtonReleased,
    MouseMoved,
    MouseScrolled,
}

pub trait Event {
    fn get_event_type(&self) -> EventType;
    fn get_handled(&self) -> bool;
    fn set_handled(&mut self, is_handled: bool);
    fn get_category_flags() -> u8;
    fn is_in_category(category: EventCategory) -> bool;
    fn get_categories() -> &'static [EventCategory];
}
