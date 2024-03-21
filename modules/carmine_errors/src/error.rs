use crate::event_dispatch_error::EventDispatchError;
use crate::user_error::UserError;
use std::fmt::Debug;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CarmineError {
    #[error("")]
    UserError(#[from] UserError),
    #[error("")]
    EventDispatchError(#[from] EventDispatchError),
}

impl CarmineError {
    pub fn user() -> CarmineError {
        CarmineError::UserError(UserError::None)
    }
    pub fn event_dispatch(msg: &'static str) -> CarmineError {
        CarmineError::EventDispatchError(EventDispatchError(msg))
    }
}
