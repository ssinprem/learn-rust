/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    let mut vec = Vec::<u64>::new();

    // check value charactor
    let mut string = String::new();
    for c in code.chars() {
        match c {
            '0'..='9' => { string.push(c)},
            ' ' => {},
            _ => return false,
        }
    }

    if string.len() <=1 {
        return false;
    }

    let is_odd = string.len() % 2;
    for (i,n) in string.chars().enumerate() {
        match n {
            '0'..='9' => {
                if i % 2 == is_odd {
                    vec.push(match n {
                        '0' => 0,
                        '1' => 2,
                        '2' => 4,
                        '3' => 6,
                        '4' => 8,
                        '5' => 1,
                        '6' => 3,
                        '7' => 5,
                        '8' => 7,
                        '9' => 9,
                        _ => panic!("invalid state")
                    })
                } else {
                    vec.push(n.to_string().parse::<u64>().unwrap());
                }
            },
            _ => { continue; }
        }
    }
    let sum = vec.iter().sum::<u64>();
    sum % 10 == 0
}
