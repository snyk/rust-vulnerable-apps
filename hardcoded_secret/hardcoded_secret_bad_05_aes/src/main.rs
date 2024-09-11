/*
[dependencies]
aes = "0.8.4"
*/

use aes::Aes192;
use aes::cipher::{
    BlockCipher, BlockEncrypt, BlockDecrypt, KeyInit,
    generic_array::GenericArray,
};
// Based on https://docs.rs/aes/latest/aes/

fn main() {
  let key = [0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23];
  let mut block = GenericArray::from([42u8; 16]);

  // Initialize cipher
  // Testing the new_from_slice
  //  HardcodedSecret VULNERABILITY HERE
  let cipher = match Aes192::new_from_slice(&key) {
      Ok(cipher) => cipher,
      Err(e) => {
          println!("Error: {:?}", e);
          return;
      }
  };

  let block_copy = block.clone();

  // Encrypt block in-place
  cipher.encrypt_block(&mut block);

  // And decrypt it back
  cipher.decrypt_block(&mut block);
  assert_eq!(block, block_copy);

  // Implementation supports parallel block processing. Number of blocks
  // processed in parallel depends in general on hardware capabilities.
  // This is achieved by instruction-level parallelism (ILP) on a single
  // CPU core, which is differen from multi-threaded parallelism.
  let mut blocks = [block; 1];

  println!("Plaintext : {:?}", blocks);

  cipher.encrypt_blocks(&mut blocks);

  println!("Ciphertext: {:?}", blocks);

  for block in blocks.iter_mut() {
      cipher.decrypt_block(block);
  }

  // `decrypt_blocks` also supports parallel block processing.
  cipher.decrypt_blocks(&mut blocks);

  for block in blocks.iter_mut() {
      cipher.encrypt_block(block);
  }

  println!("Decrypted plaintext: {:?}", blocks)

}