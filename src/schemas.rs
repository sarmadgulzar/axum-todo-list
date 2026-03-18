use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
}
