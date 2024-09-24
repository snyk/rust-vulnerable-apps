/*
[dependencies]
age = "0.9"
*/

use age::{secrecy::Secret, DecryptError, EncryptError};
use std::{io::{Read, Write}, string::FromUtf8Error};

// Based on https://docs.rs/age/latest/age/

// Here, the event graph does not join the definitions of the passphrase
// in main() to their usage in encrypt(), decrypt()
// .e. False Negatives...

fn encrypt(plaintext: &[u8], passphrase: &str) -> Result<Vec<u8>, EncryptError> {
    // Encrypt the plaintext to a ciphertext using the passphrase...
    let encrypted = {
        //  HardcodedSecret VULNERABILITY HERE
        let encryptor = age::Encryptor::with_user_passphrase(Secret::new(passphrase.to_owned()));

        let mut encrypted = vec![];
        let mut writer = encryptor.wrap_output(&mut encrypted)?;
        writer.write_all(plaintext)?;
        writer.finish()?;

        encrypted
    };

    Ok(encrypted)
}

fn decrypt(ciphertext: Vec<u8>, passphrase: &str) -> Result<Vec<u8>, DecryptError> {
    // Decrypt the ciphertext to a plaintext using the passphrase...
    let decryptor = match age::Decryptor::new(&ciphertext[..])? {
        age::Decryptor::Passphrase(d) => d,
        _ => unreachable!(),
    };

    let mut decrypted = vec![];
    // FN -> event graph does not join passphrase here with the hardcoding in the calling context
    //  HardcodedSecret VULNERABILITY HERE
    let mut reader = decryptor.decrypt(&Secret::new(passphrase.to_owned()), None)?;
    reader.read_to_end(&mut decrypted);

    Ok(decrypted)
}   

fn main() {

    let plaintext = b"Hello sneaky!";
    let passphrase = "sneaky little password";

    let ciphertext = encrypt(plaintext, passphrase).unwrap();
    let decrypted_plaintext = decrypt(ciphertext, passphrase).unwrap();

    println!("Encrypted: {:?} and decrypted with {:?} to receive plaintext {:?}", String::from_utf8(plaintext.to_vec()), passphrase, String::from_utf8(decrypted_plaintext));
}
