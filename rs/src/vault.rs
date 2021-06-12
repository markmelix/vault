use openssl::{pkey::Private, rsa::{Padding, Rsa}};
use crate::auth;
use std::str;
use crate::Result;

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
}

pub struct VaultData {
    data: Vec<u8>,
    private_key: Rsa<Private>,
}

impl VaultData {
    fn new() -> Result<Self> {
        Ok(Self {
            data: Vec::new(),
            private_key: Rsa::generate(2048)?,
        })
    }

    fn encrypt(&mut self, data: Vec<u8>) -> Result<()> {
        let mut buf = vec![0; self.private_key.size() as usize];
        self.private_key.public_encrypt(data.as_ref(), &mut buf, Padding::PKCS1)?;
        self.data = buf.to_vec();
        Ok(())
    }

    fn decrypt(&self) -> Result<Vec<u8>> {
        let mut buf = vec![0; self.private_key.size() as usize];
        self.private_key.private_decrypt(self.data.as_ref(), &mut buf, Padding::PKCS1)?;
        Ok(buf.to_vec())
    }
}
