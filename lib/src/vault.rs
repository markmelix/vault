//! Vault system.

use crate::{auth, Result};
use argon2::password_hash::SaltString;
use bincode_aes::BincodeCryptor;
use pbkdf2::pbkdf2_hmac;
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use sha2::Sha512;
use std::{
    cell::RefCell,
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    str,
};

const PBKDF2_ITERATIONS: u32 = 210000;

/// This structure provides all needed methods to interact with vaults.
#[derive(Serialize, Deserialize)]
pub struct Vault {
    /// Password hash of the vault used for vault decryption.
    password_hash: String,

    /// Salt for key generation.
    salt: Vec<u8>,

    /// Data stored inside the vault.
    data: RefCell<Vec<u8>>,
}

impl Vault {
    /// Return new Vault instance with provided password and size in bits.
    pub fn new(password: String, size: usize) -> Result<Self> {
        Ok(Self {
            password_hash: auth::password_hash(password),
            salt: SaltString::generate(&mut OsRng).as_bytes().to_vec(),
            data: RefCell::new(Vec::with_capacity(size)),
        })
    }

    /// Open file using provided path and deserialize Vault instance from that file.
    pub fn open<T>(path: T) -> Result<Self>
    where
        T: Into<PathBuf>,
    {
        let path = path.into();
        let mut file = File::open(path)?;
        let mut bytes = Vec::new();
        file.read_to_end(&mut bytes)?;
        Ok(bincode::deserialize(bytes.as_ref())?)
    }

    fn cryptor(&self, password: String) -> BincodeCryptor {
        let mut key = [0u8; 32];
        pbkdf2_hmac::<Sha512>(password.as_bytes(), &self.salt, PBKDF2_ITERATIONS, &mut key);
        bincode_aes::with_key(bincode_aes::create_key(key.to_vec()).unwrap())
    }

    /// Decrypt data stored in vault.
    /// Returns empty string if data was never encrypted.
    pub fn decrypt(&self, password: String) -> Result<String> {
        if auth::password_verify(password.clone(), self.password_hash.clone()) {
            self.cryptor(password)
                .deserialize(&mut self.data.borrow_mut())
        } else {
            Err(auth::PasswordsMismatchError.into())
        }
    }

    /// Encrypt vault using the provided data.
    /// Return PasswordsMismatchError if password hash mismatches password_hash field of the structure.
    pub fn encrypt(&mut self, password: String, data: String) -> Result<()> {
        if auth::password_verify(password.clone(), self.password_hash.clone()) {
            self.data.replace(self.cryptor(password).serialize(&data)?);
            Ok(())
        } else {
            Err(auth::PasswordsMismatchError.into())
        }
    }

    pub fn encrypt_append(&mut self, password: String, data: String) -> Result<()> {
        self.encrypt(
            password.clone(),
            format!("{}{}", self.decrypt(password)?.trim_end(), data),
        )
    }

    /// Save vault to the specified file.
    /// If file exists, remove it and create new one.
    pub fn save<T>(&self, path: T) -> Result<()>
    where
        T: Into<PathBuf>,
    {
        let path = path.into();
        if path.exists() {
            fs::remove_file(&path)?;
        }
        let mut file = File::create(path)?;
        file.write_all(bincode::serialize(self)?.as_ref())?;
        Ok(())
    }
}

pub fn bytes_to_bits(amount: usize) -> usize {
    amount * 8
}

pub fn mb_to_bits(amount: usize) -> usize {
    amount * 8000000
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vault() -> Result<()> {
        let vault_password = String::from("");
        let vault_size = bytes_to_bits(1024);
        let _vault = Vault::new(vault_password, vault_size)?;

        Ok(())
    }
}
