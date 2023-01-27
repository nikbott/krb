use magic_crypt::{new_magic_crypt, MagicCryptTrait};
use rand::distributions::{Alphanumeric, DistString};

pub fn decrypt(key: &str, data: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.decrypt_base64_to_string(data.trim()).unwrap()
}

pub fn encrypt(key: &str, data: &str) -> String {
    let mc = new_magic_crypt!(key, 256);
    mc.encrypt_str_to_base64(data.trim())
}

pub fn gen_secret() -> String {
    Alphanumeric.sample_string(&mut rand::thread_rng(), 64)
}
