use super::error::Result;

pub async fn root() -> Result<&'static str> {
    Ok("Hello, world")
}

pub async fn health() -> Result<&'static str> {
    Ok("Ok")
}
