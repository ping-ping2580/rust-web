use crate::errors::MyError;
use crate::models::course::{Course, UpdateCourse};
use sqlx::postgres::PgPool;
use sqlx::query_as;
// use sqlx::FromRow;

pub async fn get_courses_for_teacher_db(pool: &PgPool, teacher_id: i32) -> Result<Vec<Course>, MyError> {
    let rows: Vec<Course> = query_as(
        r#"SELECT * FROM course WHERE teacher_id = $1"#
    )
    .bind(teacher_id)
    .fetch_all(pool)
    .await?;

    Ok(rows)
}

pub async fn get_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    course_id: i32,
) -> Result<Course, MyError> {
    let row: Option<Course> = query_as(
        r#"SELECT * FROM course WHERE teacher_id = $1 and id = $2"#
    )
    .bind(teacher_id)
    .bind(course_id)
    .fetch_optional(pool)
    .await?;

    row.ok_or(MyError::NotFound("Course Id not found".into()))
}

pub async fn post_new_course_db(pool: &PgPool, new_course: Course) -> Result<Course, MyError> {
    let row: Course = query_as(
        r#"INSERT INTO course (teacher_id, name, description, format, structure, duration, price, language, level, time)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"#
    )
    .bind(new_course.teacher_id)
    .bind(new_course.name)
    .bind(new_course.description)
    .bind(new_course.format)
    .bind(new_course.structure)
    .bind(new_course.duration)
    .bind(new_course.price)
    .bind(new_course.language)
    .bind(new_course.level)
    .bind(new_course.time.map(|t| t.naive_utc())) // 转换为 NaiveDateTime 兼容 PostgreSQL
    .fetch_one(pool)
    .await?;

    Ok(row)
}

pub async fn delete_course_db(pool: &PgPool, teacher_id: i32, id: i32) -> Result<String, MyError> {
    let course_row = sqlx::query!(
        "DELETE FROM course where teacher_id = $1 and id = $2",
        teacher_id,
        id,
    )
    .execute(pool)
    .await?;
    Ok(format!("Deleted {:?} record", course_row))
}

pub async fn update_course_details_db(
    pool: &PgPool,
    teacher_id: i32,
    id: i32,
    update_course: UpdateCourse,
) -> Result<Course, MyError> {
    let current_course_row = query_as::<_, Course>(
        "SELECT * FROM course where teacher_id = $1 and id = $2"
    )
    .bind(teacher_id)
    .bind(id)
    .fetch_one(pool)
    .await
    .map_err(|_err| MyError::NotFound("Course Id not found".into()))?;

    let name: String = update_course.name.unwrap_or_else(|| current_course_row.name.unwrap_or_default());
    let description: String = update_course.description.unwrap_or_else(|| current_course_row.description.unwrap_or_default());
    let format: String = update_course.format.unwrap_or_else(|| current_course_row.format.unwrap_or_default());
    let structure: String = update_course.structure.unwrap_or_else(|| current_course_row.structure.unwrap_or_default());
    let duration: String = update_course.duration.unwrap_or_else(|| current_course_row.duration.unwrap_or_default());
    let level: String = update_course.level.unwrap_or_else(|| current_course_row.level.unwrap_or_default());
    let language: String = update_course.language.unwrap_or_else(|| current_course_row.language.unwrap_or_default());
    let price: String = update_course.price.unwrap_or_else(|| current_course_row.price.unwrap_or_default());

    let course_row = query_as::<_, Course>(
        "UPDATE course SET name = $1, description = $2, format = $3, structure = $4, duration = $5, price = $6, language = $7, level = $8 WHERE teacher_id = $9 AND id = $10 
         RETURNING id, teacher_id, name, time, description, format, structure, duration, price, language, level"
    )
    .bind(name)
    .bind(description)
    .bind(format)
    .bind(structure)
    .bind(duration)
    .bind(price)
    .bind(language)
    .bind(level)
    .bind(teacher_id)
    .bind(id)
    .fetch_one(pool)
    .await?;

    Ok(course_row)
}
