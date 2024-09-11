/*
[dependencies]
camellia = "0.1.0"
*/

use camellia::cipher::generic_array::GenericArray;
use camellia::cipher::{BlockDecrypt, BlockEncrypt, KeyInit};
use camellia::Camellia128;

// Based on https://docs.rs/camellia/latest/camellia/

fn main() {
  let key = GenericArray::from([0_u8; 16]);
  let mut block = GenericArray::from(['a' as u8; 16]);

  // Initialize cipher
  //  HardcodedSecret VULNERABILITY HERE
  let cipher = Camellia128::new(&key);

  let block_copy = block;

  // Encrypt block in-place
  cipher.encrypt_block(&mut block);

  // And decrypt it back
  cipher.decrypt_block(&mut block);

  println!("Encrypted ciphertext: {:?} back to {:?}", block, block_copy)
}