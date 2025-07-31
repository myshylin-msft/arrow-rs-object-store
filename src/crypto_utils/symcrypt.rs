#[cfg(any(feature = "aws", feature = "azure"))]
pub(crate) fn hmac_sha256(secret: impl AsRef<[u8]>, bytes: impl AsRef<[u8]>) -> Vec<u8> {
    symcrypt::hmac::hmac_sha256(secret.as_ref(), bytes.as_ref())
        .expect("faild to hmac 256 using symcrypt")
        .to_vec()
}

#[cfg(any(feature = "aws", feature = "gcp"))]
pub(crate) fn hex_digest(bytes: &[u8]) -> String {
    use crate::util::hex_encode;

    hex_encode(symcrypt::hash::sha256(bytes))
}
