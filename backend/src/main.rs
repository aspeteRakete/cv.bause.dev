use axum::{routing::get, Router, Json};
use serde::Serialize;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/health/alive", get(alive));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Health {
    alive: bool,
}

async fn alive() -> Json<Health> {
    let healthy = Health {
        alive: true,
    };
    Json(healthy)
}