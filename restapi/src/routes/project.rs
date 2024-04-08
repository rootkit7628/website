use std::net::SocketAddr;

use super::logging;
use crate::{
    database,
    models::project::{ProjectIN, ProjectORM, ProjectOUT},
};
use axum::{
    extract::{ConnectInfo, Json, Path},
    http::StatusCode,
    response::IntoResponse,
};
use rusqlite::Connection;

pub struct ProjectRoutes {
    conn: Connection,
}

impl ProjectRoutes {
    fn new() -> Self {
        Self {
            conn: database::connect(),
        }
    }

    pub async fn get_all(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> impl IntoResponse {
        logging(&format!("GET from {addr} /project"));
        let orm = ProjectORM::new(Self::new().conn);
        let projects = orm.get_all();
        Json(projects)
    }

    pub async fn get(
        Path(id): Path<u16>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> impl IntoResponse {
        logging(&format!("GET from {addr} /project/{id}"));
        let orm = ProjectORM::new(Self::new().conn);
        let project = orm.get(id as i64);
        match project {
            Some(p) => (StatusCode::ACCEPTED, Json(p)).into_response(),
            None => (StatusCode::NOT_FOUND, "Project not found").into_response(),
        }
    }

    pub async fn create(
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        Json(project): Json<ProjectIN>,
    ) -> impl IntoResponse {
        logging(&format!("POST from {addr} /project"));
        // Create a new project
        let orm = ProjectORM::new(Self::new().conn);
        let id = orm.insert(&project);

        Json(ProjectOUT {
            id,
            name: project.name.clone(),
            description: project.description.clone(),
        })
    }

    pub async fn update(
        Path(id): Path<i64>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
        Json(data): Json<ProjectIN>,
    ) -> impl IntoResponse {
        logging(&format!("PUT from {addr} /project/{id}"));
        let orm = ProjectORM::new(Self::new().conn);
        let project = orm.update(id, &data);
        Json(project)
    }

    pub async fn delete(
        Path(id): Path<u32>,
        ConnectInfo(addr): ConnectInfo<SocketAddr>,
    ) -> impl IntoResponse {
        logging(&format!("DELETE from {addr} /project/{id}"));
        format!("Delete project with id: {}", id)
    }
}
