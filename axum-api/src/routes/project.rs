use std::net::SocketAddr;

use axum::{extract::{Path, ConnectInfo, Json}, response::IntoResponse};
use rusqlite::Connection;
use super::logging;
use crate::database;
use crate::models::project::ProjectORM;

pub struct ProjectRoutes {
    conn: Connection
}

impl ProjectRoutes {
    fn new() -> Self {
        Self { conn: database::connect() }
    }

    pub async fn get_all(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
        logging(&format!("GET from {addr} /project"));
        let orm = ProjectORM::new(Self::new().conn);
        let projects = orm.get_all();
        Json(projects)
    }

    pub async fn get(Path(id): Path<u16>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
        logging(&format!("GET from {addr} /project/{id}"));
    }

    pub async fn create(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
        logging(&format!("POST from {addr} /project"));
        "Create a new project"
    }

    pub async fn update(Path(id): Path<u32>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
        logging(&format!("PUT from {addr} /project/{id}"));
        format!("Update project with id: {}", id)
    }

    pub async fn delete(Path(id): Path<u32>, ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
        logging(&format!("DELETE from {addr} /project/{id}"));
        format!("Delete project with id: {}", id)
    }
}
