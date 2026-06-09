use base64::{Engine, engine::general_purpose};
use hmac::{Hmac, KeyInit, Mac};
use sha2::Sha256;

pub fn compute_hmac(secret: &str, message: &str) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("HMAC can take key of any size");
    mac.update(message.as_bytes());
    let result = mac.finalize();
    general_purpose::STANDARD.encode(result.into_bytes())
}