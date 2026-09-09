#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    SpanTooLong,
    InvalidDigit(char),
}

pub fn lsp(string_digits: &str, span: usize) -> Result<u64, Error> {
    if span == 0 {
        return Ok(1);
    }
    if string_digits.len() < span {
        return Err(Error::SpanTooLong);
    }
    if let Some(char) = string_digits.chars().find(|c| !c.is_ascii_digit()) {
        return Err(Error::InvalidDigit(char));
    }

    let result = string_digits.chars().collect::<Vec<char>>()
        .windows(span)
        .map(|chars| {
            chars.iter()
            .map(|c|
                format!("{c}").parse::<u64>().unwrap()
            )
            .product()
        }).max().unwrap();


    Ok(result)
}
