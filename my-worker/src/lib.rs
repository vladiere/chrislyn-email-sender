mod config;

pub mod email;
// pub mod routes_static;
pub mod error;
pub use config::core_config;

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_email_send_ok() {
        use crate::email::send_email;
        use crate::email::EmailTemplateParams;

        let email_params = EmailTemplateParams {
            id_number: 2002080800,
            email_to: "feyonaj308@ruhtan.com".to_string(),
            exam_phase: "Midterm".to_string(),
            to_name: "John Doe".to_string(),
            course_code: "BSIT".to_string(),
            section_code: "1C".to_string(),
            avg_grade: 2.4,
            quiz: crate::email::Quiz { score: 24.0, total: 50.0, },
            assignment: crate::email::Assignment { score: 22.0, total: 30.0 },
            attendance: crate::email::Attendance { score: 31.0, total: 33.0 },
            activities: crate::email::Activities { score: 65.0, total: 74.0 },
            major_exam: crate::email::MajorExam { score: 100.0, total: 150.0 },
        };

        let _ = send_email(email_params).await;
    }
}
