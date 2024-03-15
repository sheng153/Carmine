use thiserror::Error;

#[derive(Error, derive_more::Display, Debug)]
pub struct EventDispatchError(String);
