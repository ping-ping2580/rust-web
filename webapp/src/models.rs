// use serde::{Deserialize, Serialize};

// #[derive(Serialize, Deserialize, Debug)]
// pub struct TeacherRegisterForm
// {
//     pub name: String,
//     pub imageurl: String,
//     pub profile: String,
// }

// #[derive(Serialize, Deserialize, Debug)]
// pub struct TeacherResponse
// {
//     pub id: i32,
//     pub name: String,
//     #[serde(rename = "picture_url")]
//     pub imageurl: String,
//     pub profile: String,
// }

// models.rs
use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize)]
#[derive(Deserialize, Serialize, Debug)]
pub struct TeacherResponse {
    pub id: i32,
    pub name: String,
    #[serde(rename = "picture_url")]
    pub imageurl: Option<String>, // 改为可选字段
    #[serde(rename = "profile")]
    pub profile: String,
}

#[derive(Deserialize, Serialize)]
pub struct TeacherRegisterForm {
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}