use crate::handlers::{create_task, delete_task, get_tasks, update_task};

use axum::{
    routing::{get, patch},
    Router,
};
use sqlx::postgres::PgPool;

pub fn new() -> Router<PgPool> {
    Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/tasks", get(get_tasks).post(create_task))
        .route("/tasks/:task_id", patch(update_task).delete(delete_task))
        .route("/", get(|| async { "Hello world" }))
}
