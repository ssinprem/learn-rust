pub fn encode(source: &str) -> String {
    let mut set = Vec::<(char, u32)>::new();
    let mut pre = (' ',0);

    for char in source.chars()
    {
        if char == pre.0 {
            pre.1 += 1;
        } else {
            set.push(pre);
            pre = (char,1);
        }
    }
    set.push(pre);
    set.iter().map(| (char,n) | {
        match *n {
            0 => "".to_string(),
            1 => char.to_string(),
            n => n.to_string() + char.to_string().as_str(),
        }
    }).collect::<String>()
}

pub fn decode(source: &str) -> String {
    let mut set = Vec::<(char, u32)>::new();
    let mut num = 0;
    for char in source.chars()
    {
        if char.is_numeric() {
            num *= 10;
            num += char.to_digit(10).unwrap();
        } else {
            set.push
            ((  char, 
                match num {
                    0 => 1,
                    n => n
                }
            ));
            num = 0;
        }
    }
    set.iter().flat_map(|(char,num)| {
        (0..*num).map(|_| *char)
    }).collect::<String>()
}
