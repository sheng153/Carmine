mod event;
mod event_dispatcher;

pub use event::{generate_mask, Event, EventCategory, EventType};
pub use event_dispatcher::EventDispatcher;
