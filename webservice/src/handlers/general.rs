use crate::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(
    app_state: web::Data<AppState>
)-> HttpResponse
{
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    //lock 暂时锁住 防止其他线程进行修改
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}
