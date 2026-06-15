pub fn encode(source: &str) -> String {
    source.chars().fold(Vec::<(char, u32)>::new(),
    |mut set, char | {
        let last = set.last_mut();
        match last {
            Some((pre, num))
                if *pre == char => {
                    *num += 1;
            },
            _ => {
                set.push((char,1));
            }
        } set
    }).iter().map(| (char,n) | {
        match *n {
            0 => "".to_string(),
            1 => char.to_string(),
            n => n.to_string() + char.to_string().as_str(),
        }
    }).collect::<String>()
}

pub fn decode(source: &str) -> String {
    source.chars().fold((Vec::<(char, u32)>::new(), 0),
    |(mut set, mut num) , char| {
        match char {
            c if c.is_numeric() => {
                num *= 10;
                num += c.to_digit(10).unwrap();
            }
            c => {
                set.push((c, num.max(1)));
                num = 0;
            }
        }
        (set, num)
    }).0.iter().flat_map(|(char,num)| {
        (0..*num).map(|_| *char)
    }).collect::<String>()
}
