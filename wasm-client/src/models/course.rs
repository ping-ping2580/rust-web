// use crate::{dbaccess::teacher, errors::MyError};
use actix_web::web;
use super::super::error::MyError;
use chrono::{NaiveDateTime, Utc,};
use serde::{ Deserialize, Serialize};
use std::{convert::TryFrom, slice::Windows};//与from trait冲突
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use serde_wasm_bindgen::from_value;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[derive(Serialize, Deserialize, Debug, Clone, sqlx::FromRow)]
pub struct Course
{
    //option 可空
    pub teacher_id: i32,
    pub id: i32,
    pub name: Option<String>,
    pub time: Option<NaiveDateTime>,
    // pub time: Option<PrimitiveDateTime>,

    pub description: Option<String>,
    pub format: Option<String>,
    pub structure: Option<String>,
    pub duration: Option<String>,
    pub price: Option<String>,
    pub language: Option<String>,
    pub level: Option<String>,
}

pub async fn get_courses_by_teacher(teacher_id: i32) -> Result<Vec<Course>, MyError>
{
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("http://localhost:3000/courses/{}", teacher_id);

    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Accept", "applications/json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json()?).await?;

    // let courses: Vec<Course> = json.into_serde().unwrap();
    let courses: Vec<Course> = from_value(json).map_err(|e| MyError::from(e.to_string()))?;
    // let courses: Vec<Course> = from_value(&json).map_err(|e| MyError::from(e.to_string()))?;

    Ok(courses)
}

pub async fn delete_course(teacher_id: i32, course_id: i32) -> Result<(), MyError>
{
    let mut opts = RequestInit::new();
    opts.method("DELETE");
    opts.mode(RequestMode::Cors);

    let url = format!("http://localhost:300/course/{}/{}", teacher_id, course_id);

    let request = Request::new_with_str_and_init(&url, &opts).unwrap();
    request.headers().set("Accept", "applications/json").unwrap();

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request))
        .await
        .unwrap();

    assert!(resp_value.is_instance_of::<Response>());

    let resp: Response = resp_value.dyn_into().unwrap();
    let json = JsFuture::from(resp.json().unwrap()).await.unwrap();

    let _course: Course = from_value(json).map_err(|e| MyError::from(e.to_string()))?;
    // let _course: Course = json.into_serde().unwrap();  
    // let _course: Vec<Course> = from_value(json).map_err(|e| MyError::from(e.to_string()))?;
    Ok(())
}

use js_sys::Promise;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub async fn add_course(name: String, description: String) -> Result<Promise, JsValue> 
{
    let mut opts = RequestInit::new();
    opts.method("POST");
    opts.mode(RequestMode::Cors);

    let str_json = format!(
        r#"
        {{
            "teacher_id": 1,
            "name": "{}",
            "description": "{}"
        }}
        "#,
        name,
        description
    );

    opts.body(Some(&JsValue::from_str(&str_json.as_str())));

    let url = "http://localhost:3000/courses/";
    let request = Request::new_with_str_and_init(&url, &opts)?;
    request.headers().set("Content-Type", "application/json")?;
    request.headers().set("Accept", "application/json")?;

    let window = web_sys::window().ok_or("no window exists".to_string())?;
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await;
    // assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value?.dyn_into().unwrap();

    Ok(resp.json()?)
}