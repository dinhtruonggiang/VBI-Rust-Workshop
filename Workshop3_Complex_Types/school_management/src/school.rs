use std::collections::HashMap;

pub struct School<T> {
    students: HashMap<String, T>
}


impl<T:Clone + Ord> School<T> {
    pub fn new() -> Self {
        Self { students: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, grade: T) {
        self.students.insert(name.to_string(), grade);
    }

    pub fn school_grades(&self) -> Vec<T> {
        let mut grades: Vec<T> = self.students.values().cloned().collect();
        grades.sort();
        grades.dedup();
        grades
    }

    // Do testing with looping 
    // pub fn school_grades_iter(&self) -> Vec<T> {
    //     let mut grades: Vec<T> = Vec::new();
    //     //self.students.values().cloned().collect();
    //     for value in self.students.values() {
    //         grades.push(*value);
    //     }

    //     grades.sort();
    //     grades.dedup();
    //     grades
    // }

    pub fn filter_same_grade_students(&mut self, grade: T) -> Vec<String> {
        let mut students: Vec<String> = Vec::new();        
        for (name, student_grade) in  self.students.iter() {
            if *student_grade == grade {
                students.push(name.to_string());
            }   
        }
        students.sort();
        students.dedup();
        students
    }
}