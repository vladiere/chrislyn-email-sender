#!/bin/bash

# Define the API endpoint
API_URL="http://127.0.0.1:4000/api/send-email/bulk"

# Define the JSON data
JSON_DATA='[
  {
    "id_number": 20200800,
    "email_to": "borov73706@ruhtan.com",
    "exam_phase": "Midterm",
    "to_name": "John Doe",
    "section_code": "CS101",
    "course_code": "COMP101",
    "avg_grade": 85.5,
    "quiz": {
      "score": 45.0,
      "total": 50.0
    },
    "assignment": {
      "score": 30.0,
      "total": 40.0
    },
    "attendance": {
      "score": 10.0,
      "total": 10.0
    },
    "activities": {
      "score": 25.0,
      "total": 30.0
    },
    "major_exam": {
      "score": 90.0,
      "total": 100.0
    }
  },
  {
    "id_number": 20200801,
    "email_to": "chrislynmarie2000@gmail.com",
    "exam_phase": "Midterm",
    "to_name": "Jane Doe",
    "section_code": "CS102",
    "course_code": "COMP102",
    "avg_grade": 75.0,
    "quiz": {
      "score": 40.0,
      "total": 50.0
    },
    "assignment": {
      "score": 35.0,
      "total": 40.0
    },
    "attendance": {
      "score": 9.0,
      "total": 10.0
    },
    "activities": {
      "score": 28.0,
      "total": 30.0
    },
    "major_exam": {
      "score": 82.0,
      "total": 100.0
    }
  }
]'

# Execute the curl command and process the output with jq
curl -X POST "$API_URL" \
-H "Content-Type: application/json" \
-d "$JSON_DATA" | jq "."

