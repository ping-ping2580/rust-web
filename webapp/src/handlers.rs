// use std::collections::HashMap;

use crate::errors::MyError;
use crate::models::{TeacherRegisterForm, TeacherResponse};
use actix_web::{web, Error, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize)]
struct ApiErrorResponse {
    error_message: String,
}

// pub async fn get_all_teachers(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error>
// {
//     let awc_client = awc::Client::default();

//     let res = awc_client
//         .get("http://localhost:3000/teachers/")
//         .send()
//         .await
//         .unwrap()
//         .json::<Vec<TeacherResponse>>()
//         .await
//         .unwrap();

//     let mut ctx = tera::Context::new();
//     ctx.insert("error", "");
//     ctx.insert("teachers", &res);

//     // let s = tmpl
//     //     .render("teacher.html", &ctx)
//     //     .map_err(|_| MyError::TeraError("Template error".to_string()))?;

//     let s = tmpl.render("register.html", &ctx).map_err(|e| 
//     {
//         println!("Render error details: {:?}", e);
//         MyError::TeraError("Template error".to_string())
//     })?;
        
//     Ok(HttpResponse::Ok().content_type("text/html").body(s))
// }
pub async fn get_all_teachers(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error> {
    let awc_client = awc::Client::default();
    let res = awc_client
        .get("http://localhost:3000/teachers/")
        .send()
        .await
        .unwrap()
        .json::<Vec<TeacherResponse>>()
        .await
        .unwrap();
    println!("Teachers data: {:?}", res);

    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("teachers", &res);

    let s = tmpl.render("teacher.html", &ctx)
        .map_err(|e| {
            println!("Render error details: {:?}", e);
            MyError::TeraError("Template error".to_string())
        })?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
pub async fn show_register_form(tmpl: web::Data<tera::Tera>) -> Result<HttpResponse, Error>
{
    let mut ctx = tera::Context::new();
    ctx.insert("error", "");
    ctx.insert("current_name", "");
    ctx.insert("current_image_url", "");
    ctx.insert("current_profile", "");

    let s = tmpl
        .render("register.html", &ctx)
        .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

pub async fn handle_register(
    tmpl: web::Data<tera::Tera>,
    params: web::Form<TeacherRegisterForm>,
) -> Result<HttpResponse, Error>
{
    let mut ctx = tera::Context::new();
    let s;
    if params.name == "Dave"
    {
        ctx.insert("error", "Dave already exists!");

        ctx.insert("current_name", &params.name);
        ctx.insert("current_image_url", &params.image_url);
        ctx.insert("current_profile", &params.profile);
        s = tmpl    
            .render("register.html", &ctx)
            .map_err(|_| MyError::TeraError("Template error".to_string()))?;
    }
    else 
    {
        let new_teacher = json!({
            "name": &params.name,
            "picture_url": &params.image_url,
            "profile": &params.profile
        });
        let awc_client = awc::Client::default();

        let mut res = awc_client
            .post("http://localhost:3000/teachers/")
            .send_json(&new_teacher)
            .await
            .map_err(|e| MyError::ActixError(e.to_string()))?;
        let status = res.status();
        let body = res.body().await?;

        println!("API response: {}", std::str::from_utf8(&body)?);

        if !status.is_success() {
            let error_message = serde_json::from_slice::<ApiErrorResponse>(&body)
                .map(|resp| resp.error_message)
                .unwrap_or_else(|_| "Teacher register failed".to_string());

            ctx.insert("error", &error_message);
            ctx.insert("current_name", &params.name);
            ctx.insert("current_image_url", &params.image_url);
            ctx.insert("current_profile", &params.profile);

            s = tmpl
                .render("register.html", &ctx)
                .map_err(|_| MyError::TeraError("Template error".to_string()))?;
        } else {
            let teacher_response: TeacherResponse = serde_json::from_slice(&body)?;
            s = format!("Congratulations! Your id is: {}.", teacher_response.id);
        }
    }
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
