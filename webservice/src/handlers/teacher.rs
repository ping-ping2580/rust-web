use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

pub async fn get_all_teachers(
    State(app_state): State<Arc<AppState>>
) -> Result<Json<Vec<crate::models::teacher::Teacher>>, MyError>
{
    get_all_teachers_db(&app_state.db)
    .await
    .map(Json)
}

pub async fn get_teacher_details(
    State(app_state): State<Arc<AppState>>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<crate::models::teacher::Teacher>, MyError>
{
    get_teacher_details_db(&app_state.db, teacher_id)
    .await
    .map(Json)
}

pub async fn post_new_teacher(
    State(app_state): State<Arc<AppState>>,
    Json(new_teacher): Json<CreateTeacher>,
) -> Result<Json<crate::models::teacher::Teacher>, MyError>
{
    post_new_teacher_db(&app_state.db, CreateTeacher::from(Json(new_teacher)))
    .await
    .map(Json)
}

pub async fn update_teacher_details(
    State(app_state): State<Arc<AppState>>,
    Path(teacher_id): Path<i32>,
    Json(update_teacher): Json<UpdateTeacher>,
) -> Result<Json<crate::models::teacher::Teacher>, MyError>
{
    update_teacher_details_db(
        &app_state.db,
        teacher_id,
        UpdateTeacher::from(Json(update_teacher)),
    )
    .await
    .map(Json)
}

pub async fn delete_teacher(
    State(app_state): State<Arc<AppState>>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<String>, MyError>
{
    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(Json)
}

#[cfg(test)]
mod tests
{
    use super::*;
    use actix_web::http::StatusCode;
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn get_all_teachers_success_test()
    {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is ont set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState
        {
            health_check_response: "I'm OK.".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let resp = get_all_teachers(app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_tutor_teachers_success_test()
    {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is ont set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState
        {
            health_check_response: "I'm OK.".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<i32> = web::Path::from(1);
        let resp = get_teacher_details(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // #[ignore]
    #[actix_rt::test]
    async fn post_teacher_success_test()
    {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is ont set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state: web::Data<AppState> = web::Data::new(AppState
        {
            health_check_response: "I'm OK.".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });
        let new_teacher = CreateTeacher
        {
            name: Some("Third Teacher".into()),
            picture_url: Some("http//yangxu.pro".into()),
            profile: Some("A teacher in Machine learning".into()),
        };

        let teacher_param = web::Json(new_teacher);
        let resp = post_new_teacher(teacher_param, app_state).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_teacher_success_test()
    {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is ont set.");
        let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
        let app_state = web::Data::new(AppState
        {
            health_check_response: "I'm OK.".to_string(),
            visit_count: Mutex::new(0),
            db: db_pool,
        });

        let params: web::Path<i32> = web::Path::from(1);
        let resp = delete_teacher(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}
