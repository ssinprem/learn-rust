use std::collections::VecDeque;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut vec_l = VecDeque::new();
    for val in values {
        let mut vec_s = VecDeque::new();
        let mut val = *val;
        if val == 0 {
            vec_l.push_back(0);
            continue;
        }

        while val != 0 {
            let n = (val % 128) as u8;
            val /= 128;
            if vec_s.is_empty() {
                vec_s.push_front(n);
            } else {
                vec_s.push_front( 0x80 | n);
            }
        }
        for v in vec_s {
            vec_l.push_back(v);
        }
    }
    vec_l.into_iter().collect()
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut mdio = VecDeque::new();
    let mut value = 0;
    for byte in bytes {
        if value > u32::MAX / 128 {
            return Err(Error::Overflow);
        }
        value *= 128;
        value += (byte & 0x7F) as u32;
        if byte & 0x80 == 0 {
            mdio.push_back(value);
            value = 0;
        }
    }
    if mdio.is_empty() || value != 0 {
        Err(Error::IncompleteNumber)
    } else {
        Ok(mdio.into_iter().collect())
    }

}
