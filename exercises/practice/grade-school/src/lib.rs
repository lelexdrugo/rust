use std::collections::BTreeMap;

// This annotation prevents Clippy from warning us that `School` has a
// `fn new()` with no arguments, but doesn't implement the `Default` trait.
//
// Normally, it's good practice to just do what Clippy tells you, but in this
// case, we want to keep things relatively simple. The `Default` trait is not the point
// of this exercise.
#[allow(clippy::new_without_default)]
pub struct School {
    /// u32: grade
    /// Vec<String>: vector of students name
    students : BTreeMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School{
            students : BTreeMap::new()
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.students
            .entry(grade)
            .and_modify(|vet| vet.push(String::from(student)))
            .or_insert(vec!(student.to_string()));
    }

    pub fn grades(&self) -> Vec<u32> {
        //self.students.keys().map(|&x|{x}).collect()
        let keys : Vec<u32> = self.students.keys().cloned().collect();
        return keys
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        match self.students.get(&grade){
            Some(val) => {
                let mut v = val.to_vec();
                v.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                return v;
            },
            None => vec!(),
        }
    }
}
