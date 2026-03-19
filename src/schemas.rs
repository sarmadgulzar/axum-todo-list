use serde::Deserialize;

#[derive(Deserialize)]
pub struct TodoFilters {
    pub title: Option<String>,
    pub completed: Option<bool>,
    pub limit: Option<i64>,
    pub offset: Option<i64>,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}
