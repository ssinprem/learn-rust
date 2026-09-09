
fn find_rc(len: usize) -> (usize,usize) {
    for r in 1.. {
        for c in [r,r+1] {
            if r*c >= len {
                return (r,c);
            }
        }
    }
    (0,0)
}

pub fn encrypt(input: &str) -> String {
    let chars = input.chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c.to_ascii_lowercase())
            } else {
                None
            }
        }).collect::<Vec<char>>();

    let len = chars.len();
    if len == 0 {
        return "".to_string();
    }
    let (r,c) = find_rc(len);
    let mut lines = vec!["".to_string(); c];
    chars.iter().enumerate()
    .for_each(|(n,char)| {
        let row = n % c;
        lines[row] += format!("{}", *char).as_str();
    });
    lines.iter_mut().for_each(|line| {
        while line.len() < r {
            *line += " "
        }
    });
    lines.join(" ").to_string()
}
