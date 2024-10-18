
#[derive(Debug)]
struct Student {
    name: String,
    grades: Vec<Option<f32>>
}

impl Student {
    fn new(name: String) -> Self {
        Self {
            name,
            grades: Vec::new()
        }
    }

    fn add_grade(&mut self, grade: Option<f32>) {
        self.grades.push(grade);
    }

    fn calculate_average(&self) -> Option<f32> {
        let valid_grades: Vec<f32> = self.grades.iter()
            .filter_map(|&grade| grade)//filter Some(T)
            .collect();

        if valid_grades.is_empty() {
            None
        } else {
            Some(valid_grades.iter().sum::<f32>() / valid_grades.len() as f32)
        }
    }

    fn highest_grade(&self) -> Option<f32> {
        self.grades.iter()
            .filter_map(|&g| g)
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    fn lowest_grade(&self) -> Option<f32> {
        self.grades.iter()
            .filter_map(|&g| g)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_average_with_some_none() {
        let mut student = Student::new("Bob".to_string());
        student.add_grade(Some(80.0));
        student.add_grade(None);
        student.add_grade(Some(90.0));
        assert_eq!(student.calculate_average(), Some(85.0));
    }

    #[test]
    fn test_average_all_none() {
        let mut student = Student::new("Charlie".to_string());
        student.add_grade(None);
        student.add_grade(None);
        assert_eq!(student.calculate_average(), None);
    }

    #[test]
    fn test_highest_grade() {
        let mut student = Student::new("David".to_string());
        student.add_grade(Some(85.0));
        student.add_grade(None);
        student.add_grade(Some(92.5));
        assert_eq!(student.highest_grade(), Some(92.5));
    }

    #[test]
    fn test_lowest_grade() {
        let mut student = Student::new("Eve".to_string());
        student.add_grade(Some(85.0));
        student.add_grade(None);
        student.add_grade(Some(75.0));
        assert_eq!(student.lowest_grade(), Some(75.0));
    }
}