use crate::auth;
use crate::Result;
use openssl::{
    pkey::Private,
    rsa::{Padding, Rsa},
};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    str,
};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Vault {
    password_hash: String,
    data: VaultData,
}

impl Vault {
    pub fn new(password: String) -> Result<Self> {
        Ok(Self {
            password_hash: auth::password_hash(password),
            data: VaultData::new()?,
        })
    }

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

    pub fn decrypt(&self, password: String) -> Result<String> {
        if auth::password_verify(password, self.password_hash.clone()) {
            Ok(str::from_utf8(self.data.decrypt()?.as_ref())?.to_string())
        } else {
            Err(auth::PasswordsMismatchError.into())
        }
    }

    pub fn encrypt(&mut self, password: String, data: Vec<u8>) -> Result<()> {
        if auth::password_verify(password, self.password_hash.clone()) {
            self.data.encrypt(data)?;
            Ok(())
        } else {
            Err(auth::PasswordsMismatchError.into())
        }
    }

    pub fn save<T>(&self, path: T) -> Result<()>
    where
        T: Into<PathBuf>,
    {
        let path = path.into();
        let mut file = match path.exists() {
            true => {
                fs::remove_file(&path)?;
                File::create(path)?
            },
            false => File::create(path)?,
        };
        file.write_all(bincode::serialize(self)?.as_ref())?;
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct VaultData {
    pub data: Option<Vec<u8>>,
    pub private_key: Vec<u8>,
}

impl VaultData {
    fn new() -> Result<Self> {
        Ok(Self {
            data: None,
            private_key: Rsa::generate(2048)?.private_key_to_pem()?,
        })
    }

    fn private_key(&self) -> Result<Rsa<Private>> {
        Ok(Rsa::private_key_from_pem(self.private_key.as_ref())?)
    }

    fn encrypt(&mut self, data: Vec<u8>) -> Result<()> {
        let private_key = self.private_key()?;
        let mut buf = vec![' ' as u8; private_key.size() as usize];
        private_key.public_encrypt(data.as_ref(), &mut buf, Padding::PKCS1_OAEP)?;
        self.data = Some(buf.to_vec());
        Ok(())
    }

    fn decrypt(&self) -> Result<Vec<u8>> {
        match self.data.as_ref() {
            Some(data) => {
                let private_key = self.private_key()?;
                let mut buf = vec![' ' as u8; private_key.size() as usize];
                private_key.private_decrypt(data.as_ref(), &mut buf, Padding::PKCS1_OAEP)?;
                Ok(buf.to_vec())
            }
            None => Ok(Vec::new())
        }
    }
}
