/// "Encipher" with the Atbash cipher.
pub fn encode(plain: &str) -> String {
    const M: u8 = 26;
    plain
        .chars()
        .filter_map(|c| {
            if c.is_alphabetic() {
                let mut v = c.to_ascii_lowercase() as u8 - b'a';
                v = M - v - 1;
                Some((v + b'a') as char)
            } else if c.is_numeric() {
                Some(c)
            } else {
                None
            }
        })
        .collect::<Vec<char>>()
        .chunks(5)
        .map(|c| c.iter().collect())
        .collect::<Vec<String>>()
        .join(" ")
}

/// "Decipher" with the Atbash cipher.
pub fn decode(cipher: &str) -> String {
    encode(cipher).split(" ").collect()
}
