use thiserror::Error;

#[derive(Error, Debug)]
pub enum CarmineError<T> {
    UserError(T),
}
