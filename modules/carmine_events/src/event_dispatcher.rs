use std::error::Error;

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

    pub fn dispatch<F>(&mut self, event: F) -> Result<(), EventDispatchError>
    where
        F: Fn(&T) -> bool,
    {
        let event_ret = event(self.event);
        self.event.set_handled(event_ret);
        Ok(())
    }
}
