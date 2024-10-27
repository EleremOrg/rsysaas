mod applications;
mod dtos;
mod infrastructures;
mod routes;

pub use routes::{handle_authentication, handle_initial_verification};
