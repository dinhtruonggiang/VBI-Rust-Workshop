
#[cfg(test)]
mod tests {
    use super::super::School;

    //Grade type: u32
    #[test]
    fn test_case_1_it_show_empty_students_when_school_initialize() {
        let school: School<u32> = School::new();
        assert_eq!(school.school_grades().len(), 0);
    }

    #[test]
    fn test_case_2_it_show_expected_school_grades() {
        let mut school: School<u32> = School::new();
        school.add("Lee", 2);
        school.add("Nancy", 3);

        let expected_grades = vec![&2, &3];
        let grades = school.school_grades(); 
        assert_eq!(grades.len(), expected_grades.len());
        assert_eq!(grades, expected_grades);
    }
    

    #[test]
    fn test_case_3_it_show_expected_students_with_the_same_grade() {
        let mut school: School<u32> = School::new();
        school.add("Tom", 5);
        school.add("Bob", 4);
        school.add("Alice", 4);

        let filter_grade = 4;
        let expected_students = vec!["Alice", "Bob"];
        let same_grade_students = school.filter_same_grade_students(&filter_grade);
        assert_eq!(same_grade_students.len(), expected_students.len());
        assert_eq!(same_grade_students, expected_students);
    }

    //grade type: String
    #[test]
    fn test_case_2_generic_test_it_show_expected_school_grades() {
        let mut school: School<String> = School::new();
        school.add("Lee", String::from("A+"));
        school.add("Nancy", String::from("B+"));

        let expected_grades = vec![&"A+", &"B+"];
        let grades = school.school_grades(); 
        assert_eq!(grades.len(), expected_grades.len());
        assert_eq!(grades, expected_grades);
    }
    
    #[test]
    fn test_case_3_generic_test_it_show_expected_students_with_the_same_grade() {
        let mut school: School<String> = School::new();
        school.add("Tom", String::from("A+"));
        school.add("Bob", String::from("B+"));
        school.add("Alice", String::from("C+"));
        school.add("Peter", String::from("B+"));

        let filter_grade = String::from("B+");
        let expected_students = vec!["Bob", "Peter"];
        let same_grade_students = school.filter_same_grade_students(&filter_grade);
        assert_eq!(same_grade_students.len(), expected_students.len());
        assert_eq!(same_grade_students, expected_students);
    }
}