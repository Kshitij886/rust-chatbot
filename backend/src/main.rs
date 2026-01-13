#![allow(unused)]
use tokio::{
    net::TcpListener,
};
pub mod routes;
pub mod handlers;
pub mod types;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    //start the tracer 
    tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
    let apis = routes::web_api::web_api().await;
    tracing::debug!("Server is listining on {:?}", listener);
    axum::serve(listener, apis).await.unwrap();
}
