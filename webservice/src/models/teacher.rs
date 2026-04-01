use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Teacher {
    pub id: i32,
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct CreateTeacher
{
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTeacher
{
    pub name: Option<String>,
    pub picture_url: Option<String>,
    pub profile: Option<String>,
}

impl From<Json<CreateTeacher>> for CreateTeacher 
{
    fn from(new_teacher: Json<CreateTeacher>) -> Self
    {
        CreateTeacher
        {
            name: new_teacher.name.clone(),
            picture_url: new_teacher.picture_url.clone(),
            profile: new_teacher.profile.clone(),
        }
    }
}

impl From<Json<UpdateTeacher>> for UpdateTeacher
{
    fn from(update_teacher: Json<UpdateTeacher>) -> Self
    {
        UpdateTeacher
        {
            name: update_teacher.name.clone(),
            picture_url: update_teacher.picture_url.clone(),
            profile: update_teacher.profile.clone(),
        }
    }
}

