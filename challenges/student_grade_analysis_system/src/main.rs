use std::iter::zip;

#[derive(Debug, Clone)]
struct Student {
    name: String,
    grades: Vec<f32>,
    subjects: Vec<String>,
}

struct GradeAnalyzer {
    students: Vec<Student>,
}

impl Student {
    fn new(name: String) -> Self {
        // TODO: Initialize a new student with empty grades and subjects
        Self { name, grades: Vec::new(), subjects: Vec::new() }
    }

    fn add_grade(&mut self, grade: f32, subject: String) -> Result<(), String> {
        // TODO: Add a grade and subject. Grade should be between 0 and 100
        // If subject already exists, return error
        if grade <= 0.0 || grade >= 100.0 {
           return Err("Grade Should be between 0 and 100".to_string());
        }

        if self.subjects.contains(&subject) {
            return Err("Subject already exists".to_string());
        }

        self.grades.push(grade);
        self.subjects.push(subject);
        Ok(())
    }

    fn average_grade(&self) -> Option<f32> {
        // TODO: Calculate average grade using iterator methods
        self.grades
            .iter()
            .map(|&x| x )
            .reduce(|a, b| a + b )
            .map(|sum| sum / self.grades.len() as f32)
    }

    fn failing_subjects(&self) -> Vec<&String> {
        // TODO: Return subjects where grade is less than 60
        // Use zip to pair grades with subjects
        self.subjects.iter()
            .zip(self.grades.iter())
            .filter(|(_, &grade)| grade < 60.0)
            .map(|(subject, _)| subject)
            .collect()
    }
}

impl GradeAnalyzer {
    fn new() -> Self {
        // TODO: Initialize with empty students vector
        unimplemented!()
    }

    fn add_student(&mut self, student: Student) {
        // TODO: Add student to vector
        unimplemented!()
    }

    fn get_top_students(&self) -> Vec<&Student> {
        // TODO: Return students with average grade >= 90
        // Use iterator methods
        unimplemented!()
    }

    fn get_subject_average(&self, subject: &str) -> Option<f32> {
        // TODO: Calculate average grade for a specific subject across all students
        // Use iterator methods
        unimplemented!()
    }

    fn get_grade_distribution(&self) -> Vec<(String, usize)> {
        // TODO: Return count of students in each grade range:
        // A (90-100), B (80-89), C (70-79), D (60-69), F (0-59)
        // Use iterator methods
        unimplemented!()
    }

    fn find_students_by_subject(&self, subject: &str) -> Vec<&Student> {
        // TODO: Return all students who are taking a specific subject
        // Use iterator methods
        unimplemented!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_grade() {
        let mut student = Student::new("Alice".to_string());

        // Test valid grade
        assert!(student.add_grade(85.0, "Math".to_string()).is_ok());

        // Test invalid grade
        assert!(student.add_grade(101.0, "Science".to_string()).is_err());

        // Test duplicate subject
        assert!(student.add_grade(90.0, "Math".to_string()).is_err());

        assert_eq!(student.grades.len(), 1);
        assert_eq!(student.subjects.len(), 1);
    }

    #[test]
    fn test_average_grade() {
        let mut student = Student::new("Bob".to_string());
        assert_eq!(student.average_grade(), None);

        student.add_grade(80.0, "Math".to_string()).unwrap();
        student.add_grade(90.0, "Science".to_string()).unwrap();

        assert_eq!(student.average_grade(), Some(85.0));
    }

    #[test]
    fn test_failing_subjects() {
        let mut student = Student::new("Charlie".to_string());
        student.add_grade(55.0, "Math".to_string()).unwrap();
        student.add_grade(85.0, "Science".to_string()).unwrap();

        let failing = student.failing_subjects();
        assert_eq!(failing.len(), 1);
        assert_eq!(failing[0], "Math");
    }

    #[test]
    fn test_get_top_students() {
        let mut analyzer = GradeAnalyzer::new();

        let mut student1 = Student::new("Alice".to_string());
        student1.add_grade(95.0, "Math".to_string()).unwrap();

        let mut student2 = Student::new("Bob".to_string());
        student2.add_grade(85.0, "Math".to_string()).unwrap();

        analyzer.add_student(student1);
        analyzer.add_student(student2);

        let top_students = analyzer.get_top_students();
        assert_eq!(top_students.len(), 1);
        assert_eq!(top_students[0].name, "Alice");
    }

    #[test]
    fn test_subject_average() {
        let mut analyzer = GradeAnalyzer::new();

        let mut student1 = Student::new("Alice".to_string());
        student1.add_grade(90.0, "Math".to_string()).unwrap();

        let mut student2 = Student::new("Bob".to_string());
        student2.add_grade(80.0, "Math".to_string()).unwrap();

        analyzer.add_student(student1);
        analyzer.add_student(student2);

        assert_eq!(analyzer.get_subject_average("Math"), Some(85.0));
        assert_eq!(analyzer.get_subject_average("Science"), None);
    }

    #[test]
    fn test_grade_distribution() {
        let mut analyzer = GradeAnalyzer::new();

        let mut student1 = Student::new("Alice".to_string());
        student1.add_grade(95.0, "Math".to_string()).unwrap();

        let mut student2 = Student::new("Bob".to_string());
        student2.add_grade(85.0, "Math".to_string()).unwrap();

        let mut student3 = Student::new("Charlie".to_string());
        student3.add_grade(55.0, "Math".to_string()).unwrap();

        analyzer.add_student(student1);
        analyzer.add_student(student2);
        analyzer.add_student(student3);

        let distribution = analyzer.get_grade_distribution();
        assert_eq!(distribution.len(), 5); // A, B, C, D, F

        // Find count of A grades
        let a_count = distribution.iter()
            .find(|(grade, _)| grade == "A")
            .map(|(_, count)| count)
            .unwrap();
        assert_eq!(*a_count, 1);
    }
}

fn main(){

}