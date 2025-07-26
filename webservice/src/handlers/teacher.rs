use std::panic::panic_any;
use sqlx::postgres::PgConnectOptions;
use crate::dbaccess::teacher::*;
use crate::errors::MyError;
use crate::models::teacher::{CreateTeacher, UpdateTeacher};
use crate::state::AppState;

use actix_web::{web, App, HttpResponse};

pub async fn get_all_teachers(
    app_state: web::Data<AppState>
) -> Result<HttpResponse, MyError>
{
    get_all_teachers_db(&app_state.db)
    .await
    .map(|teachers| HttpResponse::Ok().json(teachers))
}

pub async fn get_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError>
{
    let teacher_id = params.into_inner();
    get_teacher_details_db(&app_state.db, teacher_id)
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn post_new_teacher(
    new_teacher: web::Json<CreateTeacher>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, MyError>
{
    post_new_teacher_db(&app_state.db, CreateTeacher::from(new_teacher))
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn update_teacher_details(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
    update_teacher: web::Json<UpdateTeacher>,
) -> Result<HttpResponse, MyError>
{
    let teacher_id = params.into_inner();
    update_teacher_details_db(
        &app_state.db,
        teacher_id,
        UpdateTeacher::from(update_teacher),
    )
    .await
    .map(|teacher| HttpResponse::Ok().json(teacher))
}

pub async fn delete_teacher(
    app_state: web::Data<AppState>,
    params: web::Path<i32>,
) -> Result<HttpResponse, MyError>
{
    let teacher_id = params.into_inner();
    delete_teacher_db(&app_state.db, teacher_id)
        .await
        .map(|teacher| HttpResponse::Ok().json(teacher))
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