//! src/routes/mod.rs
mod health_check;
mod subscriptions;
mod greet;

pub use health_check::*;
pub use subscriptions::*;
pub use greet::*;