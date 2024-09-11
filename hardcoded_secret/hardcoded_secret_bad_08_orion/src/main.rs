/*
[dependencies]
orion = "0.17.6"
*/

use orion::auth;

// Based on https://docs.rs/orion/latest/orion/auth/index.html

fn main() {
  // There exists a shared key between the user and API server
  //  HardcodedSecret VULNERABILITY HERE
  let key = auth::SecretKey::from_slice(b"Hardcoded insecure secret!!!!!!!").unwrap();

  // User generates message and authentication tag
  let msg = "Some message.".as_bytes();
  let expected_tag = auth::authenticate(&key, msg).unwrap();

  // API server verifies the authenticity of the message with the tag
  assert!(auth::authenticate_verify(&expected_tag, &key, &msg).is_ok());
}