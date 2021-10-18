// An imaginary magical school has a new report card generation system written in Rust!
// Currently the system only supports creating report cards where the student's grade
// is represented numerically (e.g. 1.0 -> 5.5).
// However, the school also issues alphabetical grades (A+ -> F-) and needs
// to be able to print both types of report card!

// Make the necessary code changes in the struct ReportCard and the impl block
// to support alphabetical report cards. Change the Grade in the second test to "A+"
// to show that your changes allow alphabetical grades.

// Execute 'rustlings hint generics3' for hints!

pub struct ReportCard {
    pub grade: f32,
    pub student_name: String,
    pub student_age: u8,
    pub show_alphabetical: bool,
}

fn transformNumberToAlpha(grade: f32) -> String {
    match grade {
        val if (5.0..=5.5).contains(&val) => "A+".to_owned(),
        val if (4.5..=5.0).contains(&val) => "A".to_owned(),
        val if (4.0..=4.5).contains(&val) => "A-".to_owned(),
        val if (3.5..=4.0).contains(&val) => "B+".to_owned(),
        val if (3.0..=3.5).contains(&val) => "B".to_owned(),
        val if (2.5..=3.0).contains(&val) => "B-".to_owned(),
        val if (2.0..=2.5).contains(&val) => "A+".to_owned(),
        val if (1.5..=2.0).contains(&val) => "A".to_owned(),
        val if (0.0..=1.5).contains(&val) => "A-".to_owned(),
        st => st.to_string().to_owned(),
    }
}

impl ReportCard {
    pub fn print(&self) -> String {
        format!("{} ({}) - achieved a grade of {}",
            &self.student_name, &self.student_age, if (self.show_alphabetical) {transformNumberToAlpha(self.grade)} else {self.grade.to_string()})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generate_numeric_report_card() {
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Tom Wriggle".to_string(),
            student_age: 12,
            show_alphabetical: false,
        };
        assert_eq!(
            report_card.print(),
            "Tom Wriggle (12) - achieved a grade of 2.1"
        );
    }

    #[test]
    fn generate_alphabetic_report_card() {
        // TODO: Make sure to change the grade here after you finish the exercise.
        let report_card = ReportCard {
            grade: 2.1,
            student_name: "Gary Plotter".to_string(),
            student_age: 11,
            show_alphabetical: true,
        };
        assert_eq!(
            report_card.print(),
            "Gary Plotter (11) - achieved a grade of A+"
        );
    }
}
