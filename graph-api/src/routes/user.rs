use crate::models;
use axum::{extract::Extension, http::StatusCode, response::{Html, IntoResponse}, Json};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: u64,
    name: String,
    age: i32,
    address: String,
    bio: String,
    email: String,
    phone: String
}

pub(crate) async fn user() -> impl IntoResponse {
    let user = User {
            id: 1,
            name: String::from("Arleme Johnson"),
            age: 23,
            address: String::from("Antananarivo"),
            bio: String::from("Just a little annoying boy."),
            email: String::from("arleme.dev7@gmail.com"),
            phone: String::from("+261386655099")
    };

    (StatusCode::OK, Json(user))
}

pub(crate) async fn playground() -> impl IntoResponse {
    Html(playground_source(
        GraphQLPlaygroundConfig::new("/").subscription_endpoint("/ws"),
    ))
}

pub(crate)  async fn handler(req: GraphQLRequest, Extension(schema): Extension<models::user::UserSchema>) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}
