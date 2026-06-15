fn get_point (garden: &[&str], x: i32, y: i32) -> Option<char> {
    if y < 0 || y as usize >= garden.len() {
        return None;
    }
    if x < 0 || x as usize >= garden[0].len() {
        return None;
    }
    garden[y as usize].chars().nth(x as usize)
}

pub fn annotate(garden: &[&str]) -> Vec<String> {
    let mut result : Vec<String> = Vec::new();
    for y in 0..garden.len() {
        let y: i32 = y.try_into().unwrap();
        let mut row = String::new();
        for x in 0..garden[y as usize].len() {
            let x: i32 = x as i32;
            if get_point(garden, x, y) == Some(' ') {
                let mut sum = 0;
                for dx in -1..=1 {
                    for dy in  -1..=1 {     
                        if get_point(garden, x+dx, y+dy) == Some('*') {
                            sum += 1;
                        }
                    }
                }
                if sum != 0 {
                    row += &format!("{}", sum);
                } else {
                    row += &format!(" ");
                }
            } else {
                row += &format!("{:}", get_point(garden, x, y).unwrap());
            }
        }
        result.push(row);
    }
    result
}
