//! Password management.

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rand_core::OsRng;

/// Hash password using Argon2 password hashing function.
pub fn password_hash(password: String) -> String {
    let argon2 = Argon2::default();
    let salt = SaltString::generate(&mut OsRng);
    argon2
        .hash_password_simple(password.as_bytes(), salt.as_ref())
        .unwrap()
        .to_string()
}

/// Hash password and check if the hashed password matches hash.
pub fn password_verify(password: String, hash: String) -> bool {
    let argon2 = Argon2::default();
    let hash = PasswordHash::new(&hash).unwrap();
    argon2.verify_password(password.as_bytes(), &hash).is_ok()
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct PasswordsMismatchError;

impl Error for PasswordsMismatchError {}

impl fmt::Display for PasswordsMismatchError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: passwords mismatch")
    }
}
