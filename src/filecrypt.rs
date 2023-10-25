use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use aes_gcm::{
    aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
    Aes256Gcm,
    Key,
    Nonce,
};
use pbkdf2::pbkdf2_hmac_array;
use sha1::Sha1;

pub fn encrypt_file(src: &str, dst: &str, password: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut plaintext = Vec::new();
    File::open(src)?.read_to_end(&mut plaintext)?;

    let mut salt = [0u8; 12];
    OsRng.fill_bytes(&mut salt);

    let dk = pbkdf2_hmac_array::<Sha1, 32>(password, &salt, 4096);
    let key = Key::<Aes256Gcm>::from_slice(&dk);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(&salt);
    let ciphertext = cipher.encrypt(nonce, plaintext.as_slice()).unwrap();

    let mut dst_file = File::create(dst)?;
    dst_file.write_all(&ciphertext)?;
    dst_file.write_all(&salt)?;

    Ok(())
}

pub fn decrypt_file(src: &str, dst: &str, password: &[u8]) -> Result<(), Box<dyn Error>> {
    let mut ciphertext = Vec::new();
    File::open(src)?.read_to_end(&mut ciphertext)?;

    let salt = &ciphertext[ciphertext.len() - 12..];
    let dk = pbkdf2_hmac_array::<Sha1, 32>(password, salt, 4096);
    let key = Key::<Aes256Gcm>::from_slice(&dk);
    let cipher = Aes256Gcm::new(key);
    let nonce = Nonce::from_slice(salt);
    let plaintext = cipher.decrypt(nonce, &ciphertext[..ciphertext.len() - 12]).unwrap();

    File::create(dst)?.write_all(&plaintext)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{remove_file, File},
        io::Read,
    };

    use super::*;

    #[test]
    fn test_encrypt_and_decrypt_file() -> Result<(), Box<dyn Error>> {
        let src_file_path = "test_src.txt";
        let encrypted_file_path = "test_encrypted.bin";
        let decrypted_file_path = "test_decrypted.txt";
        let password = b"test";

        let mut src_file = File::create(src_file_path)?;
        src_file.write_all(b"Hello, World!")?;

        encrypt_file(src_file_path, encrypted_file_path, password)?;

        decrypt_file(encrypted_file_path, decrypted_file_path, password)?;

        let mut decrypted_contents = String::new();
        File::open(decrypted_file_path)?.read_to_string(&mut decrypted_contents)?;
        assert_eq!(decrypted_contents, "Hello, World!");

        remove_file(src_file_path)?;
        remove_file(encrypted_file_path)?;
        remove_file(decrypted_file_path)?;

        Ok(())
    }
}
