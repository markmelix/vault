pub mod auth;
mod bindings;
pub mod vault;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
