use crate::routes::user::{handler, playground, user};
use axum::{routing::get, Extension, Router};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};


mod routes;
mod models;

#[tokio::main]
async fn main() {
    let schema = Schema::build(models::user::Query, EmptyMutation, EmptySubscription);

    let router = Router::new()
        .route("/", get(playground).post(handler))
        .route("/user", get(user))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7000")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap()
}
