pub fn get_diamond(c: char) -> Vec<String> {
    let n = c as usize - 'A' as usize + 1;
    let mut vec = Vec::new();

    for i in 0..n {
        let mut str = String::new();
        let char = (b'A' + i as u8) as char;
        for j in 0..n*2-1 {
            if j == (n-i)-1 || j == (n+i)-1 {
                str += format!("{char}").as_str();
            } else {
                str += " ";
            }
        }
        vec.push(str);
    }
    let len = vec.len();
    for i in 0..len-1 {
        vec.push(vec[len-i-2].clone());
    }

    vec
}
