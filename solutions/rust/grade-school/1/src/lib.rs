use std::collections::HashSet;

#[derive(Clone,Eq, PartialEq, PartialOrd, Ord)]
pub struct School {
    students : Vec<(String, u32)>,
}

impl School {
    pub fn new() -> School {
        Self {
            students : Vec::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.students.iter().find(|s| s.0 == student).is_none() {
            self.students.push((student.to_string(), grade))
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades: Vec<u32> = self.students.iter()
            .fold(HashSet::new() ,|mut hash, s| {hash.insert(s.1); hash} )
            .iter().copied().collect();
        grades.sort();
        grades
        
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut students: Vec<String> = self.students.iter()
            .filter(| s | s.1 == grade)
            .map(|s| s.clone().0)
            .collect();
        students.sort();
        students
    }
}
