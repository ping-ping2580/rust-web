use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, Teacher, UpdateTeacher};
use sqlx::postgres::PgPool;
use sqlx::query_as;

pub async fn get_all_teachers_db(pool: &PgPool) -> Result<Vec<Teacher>, MyError> {
    let teachers: Vec<Teacher> =
        query_as("SELECT id, name, picture_url, profile FROM teacher")
            .fetch_all(pool)
            .await?;

    if teachers.is_empty() {
        return Err(MyError::NotFound("NO teachers found".into()));
    }

    Ok(teachers)
}

pub async fn get_teacher_details_db(pool: &PgPool, teacher_id: i32) -> Result<Teacher, MyError> {
    let row: Option<Teacher> = query_as(
        "SELECT id, name, picture_url, profile FROM teacher WHERE id = $1",
    )
    .bind(teacher_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(MyError::NotFound("Teacher Id not found".into()))
}

pub async fn post_new_teacher_db(
    pool: &PgPool,
    new_teacher: CreateTeacher,
) -> Result<Teacher, MyError> {
    let row: Teacher = query_as(
        "INSERT INTO teacher (name, picture_url, profile)
        VALUES ($1, $2, $3)
        RETURNING id, name, picture_url, profile",
    )
    .bind(new_teacher.name)
    .bind(new_teacher.picture_url)
    .bind(new_teacher.profile)
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn update_teacher_details_db(
    pool: &PgPool,
    teacher_id: i32,
    update_teacher: UpdateTeacher,
) -> Result<Teacher, MyError> {
    let row: Option<Teacher> = query_as(
        "SELECT id, name, picture_url, profile FROM teacher WHERE id = $1",
    )
    .bind(teacher_id)
    .fetch_optional(pool)
    .await?;

    let current = row.ok_or(MyError::NotFound("Teacher id not found".into()))?;

    let updated_name = update_teacher.name.or(current.name);
    let updated_picture_url = update_teacher.picture_url.or(current.picture_url);
    let updated_profile = update_teacher.profile.or(current.profile);

    let updated_row: Teacher = query_as(
        "UPDATE teacher
        SET name = $1, picture_url = $2, profile = $3
        WHERE id = $4
        RETURNING id, name, picture_url, profile",
    )
    .bind(updated_name)
    .bind(updated_picture_url)
    .bind(updated_profile)
    .bind(teacher_id)
    .fetch_optional(pool)
    .await?
    .ok_or(MyError::NotFound("Teacher Id not found".into()))?;

    Ok(updated_row)
}

pub async fn delete_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<String, MyError> {
    let result = sqlx::query("DELETE FROM teacher WHERE id = $1")
        .bind(teacher_id)
        .execute(pool)
        .await?;

    if result.rows_affected() == 0 {
        return Err(MyError::NotFound("Teacher Id not found".into()));
    }

    Ok(format!("Deleted {} record", result.rows_affected()))
}
