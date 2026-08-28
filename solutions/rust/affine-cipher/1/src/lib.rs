use crate::AffineCipherError::NotCoprime;
use modinverse::modinverse;

/// While the problem description indicates a return status of 1 should be returned on errors,
/// it is much more common to return a `Result`, so we provide an error type for the result here.
#[derive(Debug, Eq, PartialEq)]
pub enum AffineCipherError {
    NotCoprime(i32),
}

/// Encodes the plaintext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn encode(plaintext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    const M: i32 = 26;
    if (2..26).any(|i| a % i == 0 && M % i == 0) {
        return Err(NotCoprime(a));
    }
    let encode = plaintext
        .chars()
        .filter_map(|c| {
            if c.is_whitespace() {
                None
            } else if c.is_ascii_alphabetic() {
                let c = c.to_ascii_lowercase();
                let x = c as i32 - b'a' as i32;
                let v = (a * x + b) % M;
                Some((v as u8 + b'a') as char)
            } else if c.is_ascii_digit() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(" ");

    Ok(encode)
}

/// Decodes the ciphertext using the affine cipher with key (`a`, `b`). Note that, rather than
/// returning a return code, the more common convention in Rust is to return a `Result`.
pub fn decode(ciphertext: &str, a: i32, b: i32) -> Result<String, AffineCipherError> {
    const M: i32 = 26;
    if (2..26).any(|i| a % i == 0 && M % i == 0) {
        return Err(NotCoprime(a));
    }
    if modinverse(a, M).is_none() {
        return Err(NotCoprime(0));
    }
    let ciph = ciphertext
        .chars()
        .filter_map(|c| {
            if c.is_whitespace() {
                None
            } else if c.is_alphabetic() {
                let y = c as i32 - b'a' as i32;
                let v = (modinverse(a, M).unwrap() * (y - b)) % M;
                let v = if v < 0 { v + M } else { v };
                let char = (v as u8 + b'a') as char;
                Some(char)
            } else if c.is_numeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<String>();

    Ok(ciph)
}
