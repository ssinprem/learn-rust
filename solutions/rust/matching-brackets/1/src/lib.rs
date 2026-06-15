
// brackets
// ()
// []
// {}

// find first brackets from start
//   if found close brackets -> false
// find target brackets from next position
//   
// loop util not any brackets
pub fn brackets_are_balanced(string: &str) -> bool {
    let str : Vec<char> = string
        .chars()
        .filter(|c| "()[]{}".contains(*c))
        .collect();

    let mut stack = vec![];
    for c in str {
        match c {
            '(' | '[' | '{' => stack.push(c),
            ')' | ']' | '}' => {
                let top = stack.pop();
                match top {
                    None => return false,
                    Some('(') => { if c != ')' { return false }},
                    Some('[') => { if c != ']' { return false }},
                    Some('{') => { if c != '}' { return false }},
                    _ => panic!("invalid state")
                }
            },
            _ => panic!("invalid state")
        }
    }
    stack.is_empty()
}
