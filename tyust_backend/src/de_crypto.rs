use rand::{Rng, RngCore};
use base64::{engine::general_purpose, Engine as _};
use des::cipher::{KeyInit, BlockEncryptMut};
use des::Des;
use anyhow::{anyhow, Result};

fn generate_des_key() -> String {
    let mut key = [0u8; 8];
    rand::rng().fill_bytes(&mut key);
    general_purpose::STANDARD.encode(key)
}

pub fn get_crypto_and_password(password: &str) -> Result<(String, String)> {
    let crypto = generate_des_key(); // 假设 generate_des_key() 返回 String
    let key_bytes = general_purpose::STANDARD
        .decode(&crypto)
        .map_err(|e| anyhow!("Base64 decode failed: {}", e))?;

    let mut cipher = Des::new_from_slice(&key_bytes)
        .map_err(|e| anyhow!("Failed to create DES cipher: {:?}", e))?;

    let mut buffer = password.as_bytes().to_vec();
    let block_size = 8;
    let pad_len = block_size - (buffer.len() % block_size);
    buffer.extend(std::iter::repeat(pad_len as u8).take(pad_len));

    for chunk in buffer.chunks_mut(block_size) {
        cipher.encrypt_block_mut(chunk.into());
    }

    let password_str = general_purpose::STANDARD.encode(buffer);

    Ok((crypto, password_str))
}

#[allow(unused)]
// 生成随机 csrf key
fn generate_csrf_key(count: usize) -> String {
    let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::rng();
    (0..count)
        .map(|_| {
            let idx = rng.random_range(0..chars.len());
            chars[idx] as char
        })
        .collect()
}

#[allow(unused)]
// 生成 csrf key/value
pub fn get_csrf_key_and_value() -> (String, String) {
    let csrf_key = generate_csrf_key(32);
    let temp = general_purpose::STANDARD.encode(csrf_key.as_bytes());

    let mid = temp.len() / 2;
    let middle_crypto = format!("{}{}{}", &temp[..mid], temp, &temp[mid..]);

    let csrf_value = format!("{:x}", md5::compute(middle_crypto));

    (csrf_key, csrf_value)
}