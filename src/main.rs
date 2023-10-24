use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {

    // build our application with a route
    let app = Router::new()
        // test router
        .route("/", get(root))
        // 
        .route("/code/compile", post(compile))
        .route("/code/execute", post(execute));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, World!"
}

// basic handler that responds with a static string
async fn compile() -> &'static str {
    "Hello, World!"
}

// basic handler that responds with a static string
async fn execute() -> &'static str {
    "Hello, World!"
}

#[derive(Debug, Clone, Deserialize)]
struct ExecuteRequest {
    code: String,
}

#[derive(Debug, Clone, Serialize)]
struct ExecuteResponse {
    success: bool,
    stdout: String,
    stderr: String,
}