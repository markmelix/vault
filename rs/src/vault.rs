use flate2::{Compression, read::GzDecoder, write::GzEncoder};
use openssl::{pkey::Private, rsa::{Padding, Rsa}};
use tar::Archive;
use tempfile::tempdir;
use crate::auth;
use std::{fs::{self, File}, io::Write, path::{Path, PathBuf}, str};
use crate::Result;
use serde::{Serialize, Deserialize};

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

    pub fn from<T>(path: T) -> Result<Self>
        where T: Into<PathBuf>
    {
        let path = path.into();
        let temp_dir = tempdir()?;
        let temp_dir_path = temp_dir.path();

        let tar_gz = File::open(path)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);

        archive.unpack(temp_dir_path)?;

        let data_path = temp_dir_path.join("data.bin");
        let private_key_path = temp_dir_path.join("private_key.bin");
        let password_hash_path = temp_dir_path.join("password_hash.bin");

        let result = Ok(Self {
            password_hash: fs::read_to_string(password_hash_path)?,
            data: VaultData::from(
                      data_path,
                      private_key_path,
                  )?,
        });

        temp_dir.close()?;

        result
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
        where T: Into<PathBuf>
    {
        let path = path.into();
        let temp_dir = tempdir()?;
        let temp_dir_path = temp_dir.path();

        let mut password_hash_file = File::create(temp_dir_path.join("password_hash.bin"))?;
        let mut private_key_file = File::create(temp_dir_path.join("private_key.bin"))?;
        let mut data_file = File::create(temp_dir_path.join("data_file.bin"))?;

        password_hash_file.write_all(self.password_hash.as_bytes());
        private_key_file.write_all(self.data.private_key.as_ref());
        data_file.write_all(self.data.data.as_ref());

        let tar_gz = File::create(path)?;
        let enc = GzEncoder::new(tar_gz, Compression::default());
        let mut tar = tar::Builder::new(enc);

        tar.append_file("password_hash.bin", &mut password_hash_file);
        tar.append_file("private_key.bin", &mut private_key_file);
        tar.append_file("data.bin", &mut data_file);

        temp_dir.close()?;        
        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
pub struct VaultData {
    pub data: Vec<u8>,
    pub private_key: Vec<u8>,
}

impl VaultData {
    fn new() -> Result<Self> {
        Ok(Self {
            data: Vec::new(),
            private_key: Rsa::generate(2048)?.private_key_to_pem()?,
        })
    }

    fn from<T>(data_path: T, private_key_path: T) -> Result<Self>
        where T: AsRef<Path>
    {
        let data = fs::read(data_path)?;
        let private_key = fs::read(private_key_path)?;
        Ok(Self {
            data,
            private_key,
        })
    }

    fn private_key(&self) -> Result<Rsa<Private>> {
        Ok(Rsa::private_key_from_pem(self.private_key.as_ref())?)
    }

    fn encrypt(&mut self, data: Vec<u8>) -> Result<()> {
        let private_key = self.private_key()?;
        let mut buf = vec![0; private_key.size() as usize];
        private_key.public_encrypt(data.as_ref(), &mut buf, Padding::PKCS1)?;
        self.data = buf.to_vec();
        Ok(())
    }

    fn decrypt(&self) -> Result<Vec<u8>> {
        let private_key = self.private_key()?;
        let mut buf = vec![0; private_key.size() as usize];
        private_key.private_decrypt(self.data.as_ref(), &mut buf, Padding::PKCS1)?;
        Ok(buf.to_vec())
    }
}
