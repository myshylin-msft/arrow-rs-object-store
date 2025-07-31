#[cfg(any(feature = "aws", feature = "azure"))]
pub(crate) fn hmac_sha256(secret: impl AsRef<[u8]>, bytes: impl AsRef<[u8]>) -> Vec<u8> {
    let key = ring::hmac::Key::new(ring::hmac::HMAC_SHA256, secret.as_ref());
    ring::hmac::sign(&key, bytes.as_ref()).as_ref().collect()
}

#[cfg(any(feature = "aws", feature = "gcp"))]
pub(crate) fn hex_digest(bytes: &[u8]) -> String {
    let digest = ring::digest::digest(&ring::digest::SHA256, bytes);
    hex_encode(digest.as_ref())
}
