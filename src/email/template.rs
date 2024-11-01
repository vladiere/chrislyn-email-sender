use serde::{Deserialize, Serialize};
use time::{macros::format_description, OffsetDateTime};
use time_tz::{timezones, OffsetDateTimeExt};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Quiz {
    pub score: f32,
    pub total: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Assignment {
    pub score: f32,
    pub total: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Attendance {
    pub score: f32,
    pub total: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Activities {
    pub score: f32,
    pub total: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MajorExam {
    pub score: f32,
    pub total: f32,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct EmailTemplateParams {
    pub email_to: String,
    pub term: String,
    pub to_name: String,
    pub section_code: String,
    pub course_code: String,
    pub avg_grade: f32,
    pub quiz: Quiz,
    pub assignment: Assignment,
    pub attendance: Attendance,
    pub activities: Activities,
    pub major_exam: MajorExam,
}

pub fn template(params: EmailTemplateParams) -> String {
    let timezone = timezones::db::asia::MANILA;
    let today = OffsetDateTime::now_utc().to_timezone(timezone);
    let format = format_description!(
        "[weekday repr:short], [day] [month repr:short] [year] [hour repr:12]:[minute] [period]"
    );
    let formatted_date = today.format(&format).expect("Failed to format date");

    format!(
        r#"
    <!DOCTYPE html>
    <html lang="en">
    <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <style>
            body {{
                font-family: Arial, sans-serif;
                line-height: 1.6;
                color: #333;
                margin: 0;
                padding: 20px;
            }}
            .container {{
                max-width: 600px;
                margin: auto;
                padding: 20px;
                border: 1px solid #ddd;
                border-radius: 8px;
                background-color: #f9f9f9;
            }}
            h2 {{
                color: #007BFF;
                margin: 5px 0px;
            }}
            .grade-item {{
                margin: 10px 0;
            }}
            .footer {{
                margin-top: 30px;
                font-weight: bold;
            }}
        </style>
    </head>
    <body>

    <div class="container">
        <p>Good day, {to_name},</p>

        <p>Course Code - {course_code}</p>
        <p>Section - {section_code}</p>
        <h2>Average {term_name} grade: {avg_grade}</h2>

        <div class="grade-item">Series of Scores:</div>
        <div class="grade-item">Quiz - <strong>{quiz_score}/{quiz_total}</strong></div>
        <div class="grade-item">Assignment - <strong>{assignment_score}/{assignment_total}</strong></div>
        <div class="grade-item">Attendance - <strong>{attendance_score}/{attendance_total}</strong></div>
        <div class="grade-item">Activities - <strong>{activities_score}/{activities_total}</strong></div>
        <div class="grade-item">Major Exam - <strong>{exam_score}/{exam_total}</strong></div>

        <div class="footer">
            Best wishes,<br />
            Ms. C. Da&ntilde;o<br />
            <small>{formatted_date}</small>
        </div>
    </div>

    </body>
    </html>
    "#,
        term_name = params.term,
        avg_grade = params.avg_grade,
        to_name = params.to_name,
        course_code = params.course_code,
        section_code = params.section_code,
        quiz_score = params.quiz.score,
        quiz_total = params.quiz.total,
        assignment_score = params.assignment.score,
        assignment_total = params.assignment.total,
        attendance_score = params.attendance.score,
        attendance_total = params.attendance.total,
        activities_score = params.activities.score,
        activities_total = params.activities.total,
        exam_score = params.major_exam.score,
        exam_total = params.major_exam.total,
        formatted_date = formatted_date
    )
}
