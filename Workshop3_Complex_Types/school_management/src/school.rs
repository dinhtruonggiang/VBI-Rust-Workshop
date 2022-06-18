use std::collections::HashMap;

pub struct School<T> {
    students: HashMap<String, T>
}


impl<T:Ord> School<T> {
    pub fn new() -> Self {
        Self { students: HashMap::new() }
    }

    pub fn add(&mut self, name: &str, grade: T) {
        self.students.insert(name.to_string(), grade);
    }

    pub fn school_grades(&self) -> Vec<&T> {
        let mut grades: Vec<&T> = Vec::new();        
        for g in  self.students.values() {
            grades.push(g);            
        }
        grades.sort_unstable();
        grades.dedup();
        grades
    }

    pub fn filter_same_grade_students(&mut self, grade: &T) -> Vec<&String> {
        let mut students: Vec<&String> = Vec::new();        
        for (name, student_grade) in  self.students.iter() {
            if student_grade == grade {
                students.push(name);
            }   
        }
        students.sort_unstable();
        students.dedup();
        students
    }
}