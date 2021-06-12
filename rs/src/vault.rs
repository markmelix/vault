use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};
use crate::crypt;
use crate::auth;

pub struct Vault {
    password_hash: String,
    data: VaultData,
}

impl Vault {
    pub fn new(password: String) -> Self {
        Self {
            password_hash: auth::password_hash(password),
            data: VaultData::new(),
        }
    }

    pub fn decrypt(&self, password: String) -> Vec<u8> {
        if auth::password_verify(password, self.password_hash.clone()) {
            self.data.decrypt()
        } else {
            Vec::new()
        }
    }

    pub fn encrypt(&mut self, password: String, data: Vec<u8>) {
        if auth::password_verify(password, self.password_hash.clone()) {
            self.data.encrypt(data);
        }
    }
}

pub struct VaultData {
    data: Vec<u8>,
    private_key: RSAPrivateKey,
    public_key: Option<RSAPublicKey>,
}

impl VaultData {
    fn new() -> Self {
        Self {
            data: Vec::new(),
            private_key: crypt::private_key_gen().unwrap(),
            public_key: None,
        }
    }

    fn encrypt(&mut self, data: Vec<u8>) {
        self.data = crypt::encrypt(self.public_key(), data.as_ref()).unwrap();
    }

    fn decrypt(&self) -> Vec<u8> {
        crypt::decrypt(self.private_key(), self.data.as_ref()).unwrap()
    }

    fn private_key(&self) -> RSAPrivateKey {
        self.private_key.clone()
    }

    fn public_key(&mut self) -> RSAPublicKey {
        match self.public_key.clone() {
            Some(key) => key,
            None => {
                let key = crypt::public_key_get(&self.private_key);
                self.public_key = Some(key.clone());
                key
            }
        }
    }
}
