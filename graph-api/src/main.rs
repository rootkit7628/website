use crate::routes::user::{handler, playground, user};
use crate::analytics::metrics::{create_recorderer, track_metrics};
use axum::middleware;
use axum::{routing::get,Extension, Router};
use async_graphql::{EmptyMutation, EmptySubscription, Schema};

use std::future::ready;

mod routes;
mod models;
mod analytics;

#[tokio::main]
async fn main() {
    let schema = Schema::build(models::user::QueryRoot, EmptyMutation, EmptySubscription).finish();

    let recorder = create_recorderer();

    let router = Router::new()
        .route("/", get(playground).post(handler))
        .route("/user", get(user))
        .route("/metrics", get(move || ready(recorder.render())))
        .route_layer(middleware::from_fn(track_metrics))
        .layer(Extension(schema));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:7000")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap()
}
