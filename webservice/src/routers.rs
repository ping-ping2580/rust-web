use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};

use crate::handlers::{course::*, general::*, teacher::*};
use crate::state::AppState;

pub fn app_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check_handler))
        .route("/courses", post(post_new_course))
        .route("/courses/", post(post_new_course))
        .route("/courses/{teacher_id}", get(get_courses_for_teacher))
        .route(
            "/courses/{teacher_id}/{course_id}",
            get(get_course_detail)
                .delete(delete_course)
                .put(update_course_details),
        )
        .route("/teachers", post(post_new_teacher).get(get_all_teachers))
        .route("/teachers/", post(post_new_teacher).get(get_all_teachers))
        .route(
            "/teachers/{teacher_id}",
            get(get_teacher_details)
                .put(update_teacher_details)
                .delete(delete_teacher),
        )
        .with_state(state)
}
