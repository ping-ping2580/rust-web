use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use sqlx::error::Error as SQLxError;
use std::fmt;

#[derive(Debug, Serialize)]
pub enum MyError 
{
    DBError(String),
    ActixError(String),
    NotFound(String),    
    InvalidInput(String),
}
#[derive(Debug, Serialize)]
pub struct  MyErrorResponse
{
    error_message: String,
}

impl MyError
{
    fn error_message(&self) -> String
    {
        match self
        {
            MyError::DBError(msg) =>
            {
                println!("Database error occured: {:?}",msg);
                "Database error".into()
            }
            MyError::ActixError(msg) =>
            {
                println!("Server error occured: {:?}",msg);
                "Internal error".into()
            }
            MyError::NotFound(msg) =>
            {
                println!("Not found error occured: {:?}",msg);
                msg.into()
            }
            MyError::InvalidInput(msg) =>
            {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }

        }
    }
}

impl MyError {
    fn status_code(&self) -> StatusCode {
        match self
        {
            MyError::DBError(_msg) | MyError::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            MyError::NotFound(_msg) => StatusCode::NOT_FOUND,
            MyError::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }
}

impl IntoResponse for MyError {
    fn into_response(self) -> Response {
        let status = self.status_code();
        let body = Json(MyErrorResponse {
            error_message: self.error_message(),
        });
        (status, body).into_response()
    }
}

impl fmt::Display for MyError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(),fmt::Error>
    {
        write!(f, "{:?}", self)
    }
}

impl From<SQLxError> for MyError 
{
    fn from(err: SQLxError) -> Self
    {
        MyError::DBError(err.to_string())   
    }
}
