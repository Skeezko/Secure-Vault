use crate::crypto::CryptoManager;
use crate::models::PasswordStore;
use rand::Rng;
use std::usize;
use std::{fs, path::Path};

pub struct Storage {
    pub crypto: CryptoManager,
    pub salt: [u8; 16],
}

const STORAGE_FILE_NAME: &str = "credentials.encrypted";

impl Storage {
    pub fn load(password: &str) -> Result<(Self, PasswordStore), String> {
        if Path::new(STORAGE_FILE_NAME).exists() {
            let loaded_file =
                fs::read(STORAGE_FILE_NAME).map_err(|e| format!("Failed to read file: {}", e))?;

            if loaded_file.len() < 16 {
                return Err("File corrupted".to_string());
            }

            let (salt_slice, encrypted_data) = loaded_file.split_at(16);
            let mut salt = [0u8; 16];
            salt.copy_from_slice(salt_slice);

            let crypto = CryptoManager::new(password, &salt)?;

            let json = crypto.decrypt(encrypted_data.to_vec())?;
            let store =
                serde_json::from_str(&json).map_err(|e| format!("Failed to deserialize: {}", e))?;

            Ok((Storage { crypto, salt }, store))
        } else {
            let salt = CryptoManager::generate_salt();
            let crypto = CryptoManager::new(password, &salt)?;

            Ok((
                Storage { crypto, salt },
                PasswordStore {
                    entries: Vec::new(),
                },
            ))
        }
    }

    pub fn save(&self, pwd: PasswordStore) -> Result<(), String> {
        let serializing =
            serde_json::to_string(&pwd).map_err(|e| format!("Failed to serialize: {}", e))?;
        let encryption = self.crypto.encrypt(&serializing)?;
        let mut encrypted = self.salt.to_vec();
        encrypted.extend(encryption);
        let _ = fs::write(STORAGE_FILE_NAME, encrypted)
            .map_err(|e| format!("Failed to write: {}", e))?;

        Ok(())
    }

    pub fn generate_password(&self, len: usize) -> String {
        let charset = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*~()-_=+[]{}|;:,.<>?";
        let mut rng = rand::thread_rng();
        (0..len)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset[idx] as char
            })
            .collect()
    }
}
