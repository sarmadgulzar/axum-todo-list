use axum::response::IntoResponse;

pub async fn root() -> impl IntoResponse {
    "Hello, world"
}

pub async fn health() -> impl IntoResponse {
    "Ok"
}
