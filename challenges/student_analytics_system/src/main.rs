use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Student {
    name: String,
    age: u32,
    grades: Vec<u32>,
    major: String,
}

impl Student {
    fn new(name: String, age: u32, major: String) -> Self {
        Self { name, age, grades: Vec::new(), major }
    }

    fn add_grade(&mut self, grade: u32) {
        self.grades.push(grade);
    }

    fn average_grade(&self) -> Option<f64>{
        if self.grades.len() == 0 {
            None
        } else {
            let sum = self.grades.iter().sum::<u32>();
            Some(sum as f64 / self.grades.len() as f64)
        }
    }
}

struct StudentAnalytics {
    students: Vec<Student>,
}

impl StudentAnalytics {
    fn new() -> Self {
        Self { students: Vec::new() }
    }

    fn add_student(&mut self, student: Student) {
        self.students.push(student);
    }

    fn get_honors_students(&self) -> Vec<&Student> {
        self.students.iter()
            .filter(|&s| s.average_grade().unwrap_or(0.0) >= 90.0)  // Could handle None case
            .collect()
    }

    fn get_students_by_major(&self, major: &str) -> Vec<&Student> {
        self.students.iter()
            .filter(|&s| s.major.eq(major))
            .collect()
    }

    fn get_age_groups(&self) -> Vec<(u32, usize)> {
        self.students.iter()
            .fold(HashMap::new(), |mut acc, s| {
                acc.entry(s.age).and_modify(|counter| *counter += 1).or_insert(1);
                acc
            })
            .into_iter()
            .collect()
    }

    fn major_with_highest_average(&self) -> Option<(&str, f64)> {
        self.students.iter()
            .filter_map(|s| Some((s.major.as_str(), s.average_grade()?)))
            .fold(HashMap::new(), |mut acc, (major, grade)| {
                acc.entry(major)
                    .and_modify(|grades: &mut Vec<f64>| grades.push(grade))
                    .or_insert(vec![grade]);
                acc
            })
            .into_iter()
            .map(|(major, grades)| {
                let avg = grades.iter().sum::<f64>() / grades.len() as f64;
                (major, avg)
            })
            .max_by(|(_, avg1), (_, avg2)|
                    avg1.partial_cmp(avg2).unwrap_or(Ordering::Equal))
    }

    fn top_students_by_major(&self) -> Vec<(&str, &Student)> {
        // TODO: Find the top student (highest average grade) for each major
        // Return vector of (major, student) tuples
        // Use iterator methods
        self.students
            .iter()
            // Group by major (using fold with HashMap)
            .fold(HashMap::new(), |mut acc, student| {
                acc.entry(student.major.as_str())
                    .and_modify(|students: &mut Vec<&Student>| students.push(student))
                    .or_insert(vec![student]);
                acc
            })
            // For each major, find student with highest grade
            .into_iter()
            .filter_map(|(major, students)| {
                students
                    .into_iter()
                    .max_by(|a, b| {
                        a.average_grade()
                            .partial_cmp(&b.average_grade())
                            .unwrap_or(std::cmp::Ordering::Equal)
                    })
                    .map(|student| (major, student))
            })
            .collect()
    }
}



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_test_data() -> StudentAnalytics {
        let mut analytics = StudentAnalytics::new();

        let mut student1 = Student::new("Alice".to_string(), 20, "CS".to_string());
        student1.add_grade(95);
        student1.add_grade(92);
        analytics.add_student(student1);

        let mut student2 = Student::new("Bob".to_string(), 22, "Math".to_string());
        student2.add_grade(88);
        student2.add_grade(85);
        analytics.add_student(student2);

        let mut student3 = Student::new("Charlie".to_string(), 20, "CS".to_string());
        student3.add_grade(91);
        student3.add_grade(94);
        analytics.add_student(student3);

        analytics
    }

    #[test]
    fn test_honors_students() {
        let analytics = setup_test_data();
        let honors = analytics.get_honors_students();
        assert_eq!(honors.len(), 2);
        assert!(honors.iter().any(|s| s.name == "Alice"));
        assert!(honors.iter().any(|s| s.name == "Charlie"));
    }

    #[test]
    fn test_students_by_major() {
        let analytics = setup_test_data();
        let cs_students = analytics.get_students_by_major("CS");
        assert_eq!(cs_students.len(), 2);
    }

    #[test]
    fn test_age_groups() {
        let analytics = setup_test_data();
        let age_groups = analytics.get_age_groups();
        assert!(age_groups.contains(&(20, 2)));
        assert!(age_groups.contains(&(22, 1)));
    }

    #[test]
    fn test_major_with_highest_average() {
        let analytics = setup_test_data();
        let (major, average) = analytics.major_with_highest_average().unwrap();
        assert_eq!(major, "CS");
        assert!(average > 90.0);
    }
}