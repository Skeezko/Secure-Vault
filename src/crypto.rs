use aes_gcm::AeadCore;
use aes_gcm::aead::Aead;
use aes_gcm::{Aes256Gcm, KeyInit};
use argon2::Argon2;
use rand::RngCore;
use rand::rngs::OsRng;

pub struct CryptoManager {
    cipher: Aes256Gcm,
}

impl CryptoManager {
    pub fn new(password: &str, salt: &[u8]) -> Result<Self, String> {
        let mut key = [0u8; 32];
        Argon2::default()
            .hash_password_into(password.as_bytes(), salt, &mut key)
            .map_err(|e| format!("Key derivation failure: {}", e))?;

        let cipher = Aes256Gcm::new_from_slice(&key)
            .map_err(|e| format!("Error during initialization: {}", e))?;
        Ok(CryptoManager { cipher })
    }

    pub fn encrypt(&self, data: &str) -> Result<Vec<u8>, String> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        let ciphertext = self
            .cipher
            .encrypt(&nonce, data.as_ref())
            .map_err(|e| format!("Failed to encrypt: {}", e))?;
        let mut res: Vec<u8> = nonce.to_vec();
        res.extend_from_slice(&ciphertext);
        Ok(res)
    }

    pub fn decrypt(&self, encrypted: Vec<u8>) -> Result<String, String> {
        if encrypted.len() < 12 {
            return Err("Corrupted data".into());
        }

        let data = encrypted.split_at(12);
        let nonce = aes_gcm::Nonce::from_slice(data.0);
        let ciphertext = data.1;

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext.as_ref())
            .map_err(|_| "Failed to decrypt (Wrong password ?)".to_string())?;
        String::from_utf8(plaintext).map_err(|e| format!("Invalid UTF-8 sequence: {}", e))
    }

    pub fn generate_salt() -> [u8; 16] {
        let mut salt = [0u8; 16];
        rand::rngs::OsRng.fill_bytes(&mut salt);
        salt
    }
}
