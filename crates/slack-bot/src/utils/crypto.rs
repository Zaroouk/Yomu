use hmac::{Hmac, Mac};
use sha2::Sha256;

type HmacSha256 = Hmac<Sha256>;

pub fn verify_signature(
    signing_secret: &str,
    timestamp: &str,
    body: &str,
    slack_signature: &str,
) -> bool {

    let base = format!(
        "v0:{}:{}",
        timestamp,
        body
    );

    let mut mac =
        HmacSha256::new_from_slice(signing_secret.as_bytes())
            .unwrap();

    mac.update(base.as_bytes());

    let computed =
        format!("v0={}", hex::encode(mac.finalize().into_bytes()));

    computed == slack_signature
}