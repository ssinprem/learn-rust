
pub fn brackets_are_balanced(string: &str) -> bool {
    let str : Vec<char> = string
        .chars()
        .filter(|c| "()[]{}".contains(*c))
        .collect();

    let mut stack = vec![];
    for c in str {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            ')' | ']' | '}' => {
                let top = stack.pop();
                if top != Some(c) {
                    return false;
                }
            },
            _ => panic!("invalid state")
        }
    }
    stack.is_empty()
}
