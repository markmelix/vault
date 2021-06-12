//! Data encryption/decryption.

use rand_core::OsRng;
use rsa::errors::Result;
use rsa::{PaddingScheme, PublicKey, RSAPrivateKey, RSAPublicKey};
// use openssl::rsa::{Rsa, Padding};

/// Generates a private key.
pub fn private_key_gen() -> Result<RSAPrivateKey> {
    let bits = 2048;
    RSAPrivateKey::new(&mut OsRng, bits)
}

/// Gives a public key using the private one.
pub fn public_key_get(private_key: &RSAPrivateKey) -> RSAPublicKey {
    RSAPublicKey::from(private_key)
}

/// Encrypts a data using the public key.
pub fn encrypt(public_key: RSAPublicKey, data: &[u8]) -> Result<Vec<u8>> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    public_key.encrypt(&mut OsRng, padding, data)
}

/// Decrypts a data using the private key.
pub fn decrypt(private_key: RSAPrivateKey, enc_data: &[u8]) -> Result<Vec<u8>> {
    let padding = PaddingScheme::new_pkcs1v15_encrypt();
    private_key.decrypt(padding, enc_data)
}
