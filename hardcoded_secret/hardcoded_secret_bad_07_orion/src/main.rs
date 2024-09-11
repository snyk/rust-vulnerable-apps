/*
[dependencies]
orion = "0.17.6"
*/

use orion::{aead, errors};

// Based on https://docs.rs/orion/latest/orion/aead/index.html

fn encrypt(plaintext: &[u8], secret_key: &aead::SecretKey) -> Result<Vec<u8>, errors::UnknownCryptoError> {
    let ciphertext = aead::seal(secret_key, plaintext)?;
    Ok(ciphertext)
}

fn decrypt(ciphertext: &[u8], secret_key: &aead::SecretKey) -> Result<Vec<u8>, errors::UnknownCryptoError> {
    let decrypted_data = aead::open(secret_key, ciphertext)?;
    Ok(decrypted_data)
}

fn main() {
  let key_bytes = [3; 32];
  //  HardcodedSecret VULNERABILITY HERE
  let key = match aead::SecretKey::from_slice(&key_bytes) {
      Ok(key) => key,
      Err(err) => {
          println!("Error: Key could not be created: {:?}", err);
          return;
      }
  };


  let ciphertext = match encrypt(b"Some secret information", &key) {
      Ok(ciphertext) => Some(ciphertext),
      Err(err) => {
          println!("Error: Encryption failed: {:?}", err);
          return
      }
  };

  let decrypted_data = decrypt(&ciphertext.unwrap(), &key); 

  println!("Decrypted plaintext is {:?}", decrypted_data);
}

