pub mod auth;
pub mod vault;
pub mod bindings;

pub use vault::Vault;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
