/*
[dependencies]
age = "0.9"
*/

use age::{self, secrecy};
use std::{io::{Read, Write}, string::FromUtf8Error};

// Based on https://docs.rs/age/latest/age/

// Similar to 02_bad but using longer fully qualified names/less specific imports
// More specific import than 10_bad
fn encrypt(plaintext: &[u8]) -> Result<Vec<u8>, age::EncryptError> {
    // Encrypt the plaintext to a ciphertext using the passphrase...
    let encrypted = {
        //  HardcodedSecret VULNERABILITY HERE
        let encryptor = age::Encryptor::with_user_passphrase(secrecy::Secret::new("this is not a good passphrase".to_owned()));

        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(plaintext)?;
        writer.finish()?;

        encrypted
    };

    Ok(encrypted)
}

fn decrypt(ciphertext: Vec<u8>) -> Result<Vec<u8>, age::DecryptError> {
    // Decrypt the ciphertext to a plaintext using the passphrase...
    let decryptor = match age::Decryptor::new(&ciphertext[..])? {
        age::Decryptor::Passphrase(d) => d,
        _ => unreachable!(),
    };

    let mut decrypted = vec![];
    //  HardcodedSecret VULNERABILITY HERE
    let mut reader = decryptor.decrypt(&secrecy::Secret::new("this is not a good passphrase".to_string()), None)?;
    reader.read_to_end(&mut decrypted);

    Ok(decrypted)
}   

fn main() {

    let plaintext = b"Hello world!";

    let ciphertext = encrypt(plaintext).unwrap();
    let decrypted_plaintext = decrypt(ciphertext).unwrap();

    println!("Encrypted: {:?} and received plaintext {:?}", String::from_utf8(plaintext.to_vec()), String::from_utf8(decrypted_plaintext));
}
