use carmine_errors::CarmineError;

use crate::event::Event;

pub struct EventDispatcher<'a, T>
where
    T: Event,
{
    event: &'a mut T,
}

impl<'a, T> EventDispatcher<'a, T>
where
    T: Event,
{
    pub fn new(event: &'a mut T) -> EventDispatcher<'a, T> {
        EventDispatcher { event }
    }

    pub fn dispatch<F>(&mut self, event: F) -> Result<(), CarmineError>
    where
        F: Fn(&T) -> bool,
    {
        let event_ret = event(self.event);
        self.event.set_handled(event_ret);
        Err(CarmineError::event_dispatch("Event Dispatch Error"))
    }
}
