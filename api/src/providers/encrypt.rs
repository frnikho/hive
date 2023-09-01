use magic_crypt::{new_magic_crypt, MagicCryptTrait, MagicCrypt256, MagicCryptError};

#[derive(Debug, Clone)]
pub struct EncryptProvider {
    manager: MagicCrypt256,
}

impl EncryptProvider {
    pub fn new(encrypt_key: String) -> Self {
        EncryptProvider { manager: new_magic_crypt!(encrypt_key, 256) }
    }

    pub fn encrypt(&self, value: &str) -> String {
        self.manager.encrypt_str_to_base64(value)
    }

    pub fn decrypt(&self, value: &str) -> Result<String, MagicCryptError> {
        self.manager.decrypt_base64_to_string(value).map_err(|x| x.into())
    }
}