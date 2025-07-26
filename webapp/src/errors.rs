use actix_web::{error, http::StatusCode, HttpResponse, Result};
use serde::Serialize;
use std::{fmt};

#[allow(dead_code)]// NotFound 没有使用但是不想警告
#[derive(Debug, Serialize)]
pub enum MyError 
{
    ActixError(String),
    NotFound(String),
    TeraError(String),    
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse
{
    error_msg: String,
}

impl std::error::Error for MyError {}

impl MyError
{
    fn error_response(&self) -> String
    {
        match self
        {
            MyError::ActixError(msg) => 
            {
                println!("Server never occurred: {:?}", msg);
                "Internal server error".into()
            }
            MyError::TeraError(msg) => 
            {
                println!("Error in rendering the template {:?}", msg);
                msg.into()
            }
            MyError::NotFound(msg) => 
            {
                println!("Not found error occured: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl error::ResponseError for MyError 
{
    fn status_code(&self) -> StatusCode
    {
        match self
        {
            MyError::ActixError(_msg) | MyError::TeraError(_msg) => 
            {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> HttpResponse
    {
        HttpResponse::build(self.status_code()).json(MyErrorResponse
        {
            error_msg: self.error_response(),
        })
    }
}

// 定义一个类型如何被格式化为字符串
impl fmt::Display for MyError
{
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error>
    {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for MyError
{
    fn from(err: actix_web::error::Error) -> Self
    {
        MyError::ActixError(err.to_string())
    }
}