use base64::{engine::general_purpose, Engine as _};

pub fn base64_encode(data: &[u8]) -> String {
    let b64 = general_purpose::STANDARD.encode(data);

    b64
}

pub fn base64_decode(data: &str) -> Vec<u8> {
    let decoded = general_purpose::STANDARD.decode(data.as_bytes()).unwrap();

    decoded
}
