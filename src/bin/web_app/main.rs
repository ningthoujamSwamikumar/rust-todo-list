mod model;

use std::{io, net::SocketAddr};

use axum::{
    Json, Router,
    extract::Path,
    routing::{delete, get, patch, post, put},
};
use model::{Todo, TodoState};
use serde_json::Value;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    //define routes
    let app = Router::new()
        .route("/hello", get(hello_world))
        .route("/add", get(add_todo))
        .route("/update/:id", put(update_todo))
        .route("/patch/:id", patch(update2_todo))
        .route("/delete/:id", delete(delete_todo))
        .route("/clear", post(clear_todo))
        .route("/show/:id", get(show_single_todo))
        .route("/show", get(show_todo));

    //bind to localhost:3000
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    //run server
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn show_todo() -> Json<Value> {
    todo!()
}

async fn show_single_todo(Path(id): Path<i32>) -> Json<Value> {
    todo!()
}

async fn clear_todo() -> Json<Value> {
    todo!()
}

async fn delete_todo(Path(id): Path<i32>) -> Json<Value> {
    todo!()
}

async fn update2_todo(Json(payload): Json<serde_json::Value>) -> Json<Value> {
    todo!()
}

async fn update_todo(Json(payload): Json<serde_json::Value>) -> Json<Value> {
    todo!()
}

async fn add_todo(Json(payload): Json<serde_json::Value>) -> Json<Value> {
    todo!()
}

async fn hello_world() -> &'static str {
    "Hello, World!"
}
