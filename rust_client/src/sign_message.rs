extern crate crypto;
extern crate secp256k1;

use sign_message::secp256k1::{Message, SecretKey, Signature};
use sign_message::crypto::digest::Digest;
use sign_message::crypto::sha2::Sha256;

pub fn sign_message(message_to_hash: String, sk: String) {
    let mut sha = Sha256::new();
    sha.input_str(&message_to_hash);
    let message_hashed = &sha.result_str();
    println!("hash: {}\n",message_hashed);
    let hex_sk = hex::decode(sk);
    let hex_message_hash = hex::decode(message_hashed);
    let secret_key = SecretKey::parse_slice(&hex_sk.unwrap()).expect("32 bytes, within curve order");
    let message = Message::parse_slice(&hex_message_hash.unwrap()).expect("32 bytes");
    let (sig, recovery_id) = secp256k1::sign(&message, &secret_key).unwrap();
    let v = recovery_id.serialize() + 27;
    let r = hex::encode(sig.r.b32());
    let s = hex::encode(sig.s.b32());
    println!("v: {:?}", v);
    println!("r: {}, s: {}", r, s);
}
