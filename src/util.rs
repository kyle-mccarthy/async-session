use rand::RngCore;

/// generates a random cookie value
pub fn generate_cookie<const LEN: usize>() -> String {
    let mut key = vec![0u8; LEN];
    rand::thread_rng().fill_bytes(&mut key);
    base64::encode(key)
}

/// get the session's id from the cookie
pub fn get_id_from_cookie(cookie: &str) -> Result<String, base64::DecodeError> {
    let decoded = base64::decode(cookie)?;
    let hash = blake3::hash(&decoded);
    Ok(base64::encode(&hash.as_bytes()))
}
