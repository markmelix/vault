//! Vault system.

use crate::{auth, Result};
use openssl::{
    pkey::Private,
    rsa::{Padding, Rsa},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    str,
};

/// This structure provides all needed methods to interact with vaults.
#[derive(Serialize, Deserialize)]
pub struct Vault {
    /// Password hash of the vault used for vault decryption.
    password_hash: String,

    /// Data stored inside the vault.
    data: VaultData,
}

impl Vault {
    /// Return new Vault instance.
    pub fn new(password: String) -> Result<Self> {
        Ok(Self {
            password_hash: auth::password_hash(password),
            data: VaultData::new()?,
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

    /// Decrypt data stored in vault.
    /// Returns empty string if data was never encrypted.
    pub fn decrypt(&self, password: String) -> Result<String> {
        if auth::password_verify(password, self.password_hash.clone()) {
            Ok(str::from_utf8(self.data.decrypt()?.as_ref())?.to_string())
        } else {
            Err(auth::PasswordsMismatchError.into())
        }
    }

    /// Encrypt vault using the provided data.
    /// Return PasswordsMismatchError if password hash mismatches password_hash field of the structure.
    pub fn encrypt(&mut self, password: String, data: Vec<u8>) -> Result<()> {
        if auth::password_verify(password, self.password_hash.clone()) {
            self.data.encrypt(data)?;
            Ok(())
        } else {
            Err(auth::PasswordsMismatchError.into())
        }
    }

    /// Save vault to the specified file.
    /// If file exists, remove it and create new one.
    pub fn save<T>(&self, path: T) -> Result<()>
    where
        T: Into<PathBuf>,
    {
        let path = path.into();
        if !path.exists() {
            fs::remove_file(&path)?;
        }
        let mut file = File::create(path)?;
        file.write_all(bincode::serialize(self)?.as_ref())?;
        Ok(())
    }
}

/// Data stored inside vault.
#[derive(Serialize, Deserialize)]
struct VaultData {
    data: Option<Vec<u8>>,
    private_key: Vec<u8>,
}

impl VaultData {
    /// Generate new private key and return new instance of this structure.
    fn new() -> Result<Self> {
        Ok(Self {
            data: None,
            private_key: Rsa::generate(2048)?.private_key_to_pem()?,
        })
    }

    /// Return private key.
    fn private_key(&self) -> Result<Rsa<Private>> {
        Ok(Rsa::private_key_from_pem(self.private_key.as_ref())?)
    }

    /// Encrypt the provided data using the public key.
    fn encrypt(&mut self, data: Vec<u8>) -> Result<()> {
        let private_key = self.private_key()?;
        let mut buf = vec![' ' as u8; private_key.size() as usize];
        private_key.public_encrypt(data.as_ref(), &mut buf, Padding::PKCS1_OAEP)?;
        self.data = Some(buf.to_vec());
        Ok(())
    }

    /// Decrypt the encrypted data stored in the structure using the private key.
    /// If there's no decrypted data stored, return empty bytes vector.
    fn decrypt(&self) -> Result<Vec<u8>> {
        match self.data.as_ref() {
            Some(data) => {
                let private_key = self.private_key()?;
                let mut buf = vec![' ' as u8; private_key.size() as usize];
                private_key.private_decrypt(data.as_ref(), &mut buf, Padding::PKCS1_OAEP)?;
                Ok(buf.to_vec())
            }
            None => Ok(Vec::new()),
        }
    }
}
