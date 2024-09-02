use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub(crate) struct TaskRow {
    pub(crate) task_id: i32,
    pub(crate) name: String,
    pub(crate) priority: Option<i32>,
}

#[derive(Deserialize)]
pub(crate) struct CreateTaskReq {
    pub(crate) name: String,
    pub(crate) priority: Option<i32>,
}

#[derive(Serialize)]
pub(crate) struct CreateTaskRow {
    pub(crate) task_id: i32,
}

#[derive(Deserialize)]
pub struct UpdateTaskReq {
    pub(crate) name: Option<String>,
    pub(crate) priority: Option<i32>,
}
