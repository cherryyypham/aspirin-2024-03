/*!
    university.rs

    Manage student data at a university and calculate:
    * The average GPA of non-first-year students (`get_average_gpa`).
    * The number of students in a given class year with a GPA above the average GPA (`get_num_excel_students_for_class`).
    * The class year with the highest number of students exceeding the average GPA (`get_best_class`).
*/

#[derive(PartialEq, Clone, Copy, Debug)]

/// Represents the academic year of a student.
enum ClassYear {
    Senior,
    Junior,
    Sophomore,
    FirstYear,
}
/// Represents a student with a name, class year, and GPA.
struct Student {
    name: &'static str,
    class_year: ClassYear,
    gpa: f32,
}

/// A constant array of `Student` instances representing students at Olin College.
const OLIN_STUDENTS: [Student; 8] = [
    Student {
        name: "Alice",
        class_year: ClassYear::Senior,
        gpa: 3.9,
    },
    Student {
        name: "Foo",
        class_year: ClassYear::Sophomore,
        gpa: 2.3,
    },
    Student {
        name: "Bar",
        class_year: ClassYear::Junior,
        gpa: 3.9,
    },
    Student {
        name: "Ralph",
        class_year: ClassYear::Senior,
        gpa: 3.1,
    },
    Student {
        name: "Ayush",
        class_year: ClassYear::Senior,
        gpa: 0.0,
    },
    // New students
    Student {
        name: "Anna",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Hannah",
        class_year: ClassYear::FirstYear,
        gpa: 4.0,
    },
    Student {
        name: "Lorin",
        class_year: ClassYear::Junior,
        gpa: 3.6,
    },
];

/// Calculates the average GPA of all students except first-year students.
///
/// # Returns
///
/// Returns the average GPA as a `f32`. If there are no students other than first-year students,
/// returns `0.0`.
fn get_average_gpa() -> f32 {
    let mut total_gpa = 0.0;
    let mut count = 0;

    for student in OLIN_STUDENTS.iter() {
        if student.class_year != ClassYear::FirstYear {
            total_gpa += student.gpa;
            count += 1;
        }
    }

    if count == 0 {
        return 0.0;
    }

    total_gpa / count as f32
}

/// Calculates the number of students in a given class year with a GPA above the average GPA.
///
/// # Parameters
///
/// * `class_year`: The `ClassYear` to filter students by.
///
/// # Returns
///
/// Returns the number of students in the specified class year with a GPA above the average GPA as a `u32`.
fn get_num_excel_students_for_class(class_year: ClassYear) -> u32 {
    let average_gpa = get_average_gpa();
    OLIN_STUDENTS
        .iter()
        .filter(|student| student.class_year == class_year && student.gpa > average_gpa)
        .count() as u32
}

/// Determines the class year with the most students exceeding the average GPA.
///
/// # Returns
///
/// Returns the `ClassYear` with the most students having a GPA above the average GPA.
fn get_best_class() -> ClassYear {
    let class_years = [ClassYear::Senior, ClassYear::Junior, ClassYear::Sophomore];
    let mut best_class = ClassYear::Senior;
    let mut max_excelling = 0;

    for &class_year in class_years.iter() {
        let excelling_students = get_num_excel_students_for_class(class_year);
        if excelling_students > max_excelling {
            max_excelling = excelling_students;
            best_class = class_year;
        }
    }

    best_class
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::university::{
        get_average_gpa, get_best_class, get_num_excel_students_for_class, ClassYear,
    };

    #[test]
    fn test_get_average_gpa() {
        assert!(approx_eq!(f32, get_average_gpa(), 2.8))
    }

    #[test]
    fn test_get_num_excel_students_for_class() {
        assert_eq!(get_num_excel_students_for_class(ClassYear::Sophomore), 0);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Junior), 2);
        assert_eq!(get_num_excel_students_for_class(ClassYear::Senior), 2);
    }

    #[test]
    fn test_get_best_class() {
        assert_eq!(get_best_class(), ClassYear::Senior);
    }
}
