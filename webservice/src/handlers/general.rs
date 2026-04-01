use crate::state::AppState;
use axum::{
    extract::State,
    Json,
};
use std::sync::Arc;

pub async fn health_check_handler(
    State(app_state): State<Arc<AppState>>
)-> Json<String>
{
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    //lock 暂时锁住 防止其他线程进行修改
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    Json(response)
}
