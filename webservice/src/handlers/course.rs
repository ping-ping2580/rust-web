use crate::dbaccess::course::*;
use crate::errors::MyError;
use crate::state::AppState;
use axum::{
    extract::{Path, State},
    Json,
};
use std::sync::Arc;

use crate::models::course::{CreateCourse, Course, UpdateCourse};

pub async fn post_new_course(
    State(app_state): State<Arc<AppState>>,
    Json(new_course): Json<CreateCourse>,
) -> Result<Json<Course>, MyError>
{
    post_new_course_db(&app_state.db, Json(new_course).try_into()?)
    .await
    .map(Json)
}

pub async fn get_courses_for_teacher(
    State(app_state): State<Arc<AppState>>,
    Path(teacher_id): Path<i32>,
) -> Result<Json<Vec<Course>>,MyError>
{
    get_courses_for_teacher_db(&app_state.db, teacher_id)
        .await
        .map(Json)
    
}

pub async fn get_course_detail(
    State(app_state): State<Arc<AppState>>,
    Path((teacher_id, course_id)): Path<(i32, i32)>,
) -> Result<Json<Course>,MyError>
{
    get_course_details_db(&app_state.db, teacher_id, course_id)
    .await
    .map(Json)
}

pub async fn delete_course(
    State(app_state): State<Arc<AppState>>,
    Path((teacher_id, course_id)): Path<(i32, i32)>,
) -> Result<Json<String>,MyError>
{
    delete_course_db(&app_state.db, teacher_id, course_id)
    .await
    .map(Json)
}

pub async fn update_course_details( 
    State(app_state): State<Arc<AppState>>,
    Path((teacher_id, course_id)): Path<(i32, i32)>,
    Json(update_course): Json<UpdateCourse>,
) -> Result<Json<Course>,MyError>
{
    update_course_details_db(
        &app_state.db,
        teacher_id,
        course_id,
        Json(update_course).into(),
    )
    .await
    .map(Json)
}

#[cfg(test)]
mod tests
{
    use super::*;
    use actix_web::{http::StatusCode, ResponseError};
    use std::{f32::consts::E, sync::Mutex};
    use dotenv::dotenv;
    use sqlx::postgres::PgPoolOptions;
    use std::env;
    
    #[actix_rt::test]
    async fn post_course_test()
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

        let course = web::Json(CreateCourse
        {
            teacher_id: Some(1),
            name: Some("Course name changed".into()),
            description: Some("This is another test course".into()),
            format: Some("线上直播".into()),
            structure: Some("模块拆解".into()),
            duration: Some("6周".into()),
            price: Some("269".into()),
            language: Some("Chinese".into()),
            level: Some("Intermediate".into()),            
        });
        
        let resp = post_new_course(course, app_state).await.unwrap();
        // println!("Response status: {:?}", resp.status());
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_all_courses_success()
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

        let teacher_id: web::Path<i32> = web::Path::from(1);
        let resp = get_courses_for_teacher(app_state, teacher_id).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success()
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

        let params: web::Path<(i32, i32)> = web::Path::from((1,1));
        let resp = get_course_detail(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_fail()
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

        let params: web::Path<(i32, i32)> = web::Path::from((1,100));
        let resp = get_course_detail(app_state, params).await;
        match resp 
        {
            Ok(_) => println!("Something wrong.."),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }

    #[actix_rt::test]
    async fn update_course_success()
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

        let update_course = UpdateCourse
        {
            name: Some("Course name changed".into()),
            description: Some("This is another test course".into()),
            format: None,
            level: Some("Intermediate".into()),
            price: None,
            duration: None,
            language: Some("Chinese".into()),
            structure: None,
        };
        let params: web::Path<(i32, i32)> = web::Path::from((3,3));
        let update_param = web::Json(update_course);
        let resp = update_course_details(app_state, update_param, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn delete_course_success()
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

        let params: web::Path<(i32, i32)> = web::Path::from((1,3));
        let resp = delete_course(app_state, params).await.unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }

    // #[actix_rt::test]
    // async fn delete_course_success()
    // {
    //     dotenv().ok();
    //     let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is ont set.");
    //     let db_pool = PgPoolOptions::new().connect(&db_url).await.unwrap();
    //     let app_state = web::Data::new(AppState
    //     {
    //         health_check_response: "I'm OK.".to_string(),
    //         visit_count: Mutex::new(0),
    //         db: db_pool,
    //     });

    //     let params: web::Path<(i32, i32)> = web::Path::from((1,3));
    //     let resp = delete_course(app_state, params).await.unwrap();
    //     assert_eq!(resp.status(), StatusCode::OK);
    // }

    #[actix_rt::test]
    async fn delete_course_fail()
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

        let params: web::Path<(i32, i32)> = web::Path::from((1,101));
        let resp = delete_course(app_state, params).await;
        match resp 
        {
            Ok(_) => println!("Something wrong"),
            Err(err) => assert_eq!(err.status_code(), StatusCode::NOT_FOUND)
        }
    }
} 
