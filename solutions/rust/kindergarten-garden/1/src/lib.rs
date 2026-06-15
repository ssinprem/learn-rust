const STUDENT_LIST : [&str; 12] = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"];
const PLANTS : [(char, &str); 4] = [ ('G', "grass"), ('C', "clover"), ('R', "radishes"), ('V', "violets") ];

pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index = STUDENT_LIST.iter().position(|x| **x == *student);
    let mut list = vec![];
    if index.is_none() {
        return vec![];
    }
    let index = index.unwrap();

    let lines = diagram.split("\n");
    for l in lines {
        list.push(l.chars().nth(index*2).unwrap());
        list.push(l.chars().nth(index*2+1).unwrap());
    }
    list.iter().map(|f| {
        PLANTS.iter().find(|(c, _s)| c == f).unwrap().1
    }).collect()
}
