use std::fmt::Debug;

#[derive(Debug)]
pub enum CRUDError {
    NotFound,
    MaxRetry,
    WrongParameters,
    Write,
    Delete,
    JsonError,
}
