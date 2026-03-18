mod handlers;
mod router;
#[cfg(test)]
mod tests;

use std::net::Ipv4Addr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = router::create_router();

    let addr = Ipv4Addr::new(127, 0, 0, 1);
    let port = 8080;
    let listner = TcpListener::bind((addr, port)).await.unwrap();

    axum::serve(listner, app).await.unwrap();
}
