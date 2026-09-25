pub mod auth;
pub mod bindings;
pub mod vault;

pub use vault::Vault;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
