use crate::errors::MyError;
use axum::Json;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    pub teacher_id: Option<i32>,
    pub id: Option<i32>,
    pub name: Option<String>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub time: Option<DateTime<Utc>>, // 改回 chrono::DateTime<Utc>

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<String>,
    pub language: Option<String>,
    pub level: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub teacher_id: Option<i32>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<String>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl TryFrom<Json<CreateCourse>> for Course {
    type Error = MyError;

    fn try_from(course_json: Json<CreateCourse>) -> Result<Self, Self::Error> {
        let create_course = course_json.0;
        Ok(Course {
            teacher_id: create_course.teacher_id,
            id: Some(0),
            name: create_course.name,
            time: Some(Utc::now()), // 使用 DateTime<Utc>
            description: create_course.description,
            format: create_course.format,
            structure: create_course.structure,
            duration: create_course.duration,
            price: create_course.price,
            language: create_course.language,
            level: create_course.level,
        })
    }
}

impl TryFrom<Json<CreateCourse>> for CreateCourse {
    type Error = MyError;
    fn try_from(course: Json<CreateCourse>) -> Result<Self, Self::Error> {
        Ok(CreateCourse {
            teacher_id: course.teacher_id,
            name: course.name.clone(),
            description: course.description.clone(),
            structure: course.structure.clone(),
            format: course.format.clone(),
            duration: course.duration.clone(),
            price: course.price.clone(),
            language: course.language.clone(),
            level: course.level.clone(),
        })
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub name: Option<String>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<String>,
    pub language: Option<String>,
    pub level: Option<String>,
}

impl From<Json<UpdateCourse>> for UpdateCourse {
    fn from(course: Json<UpdateCourse>) -> Self {
        UpdateCourse {
            name: course.name.clone(),
            description: course.description.clone(),
            structure: course.structure.clone(),
            format: course.format.clone(),
            duration: course.duration.clone(),
            price: course.price.clone(),
            language: course.language.clone(),
            level: course.level.clone(),
        }
    }
}
