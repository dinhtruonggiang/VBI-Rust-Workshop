mod school;
mod test;

use school::School;


fn main() {
    //PLEASE RUN: cargo test 
    //All test cases are in test.rs file
}


// fn main() {
//     These cases are already in test.rs file    
//     let mut school: School<u32> = School::new();
//     school.add("Peter", 10);
//     school.add("Alice", 2);
//     school.add("Bob", 2);
//     let grades = school.school_grades();
//     println!("School grades: {:?}", grades);

//     let same_grade_students = school.filter_same_grade_students(&2);
//     println!("Same grade students: {:?}", same_grade_students);
// }
