use actix_web::{get, post, http::StatusCode, web::{self, Json}, Responder};
use serde::{Deserialize, Serialize};

use crate::routes::logging;

#[derive(Serialize, Deserialize)]
struct Project {
    id: i32,
    name: String,
    description: String
}

impl Project {
    fn new(id: i32, name: String, description: String) -> Self {
        Self {
            id,
            name,
            description
        }
    }
}

#[get("/project")]
pub async fn index() -> impl Responder {
    logging("GET /project");
    let response = Project::new(1, "Project 1".to_string(), "This is project 1".to_string());
    (Json(response), StatusCode::OK)
}

#[post("/project/create")]
pub async fn create(project: Json<Project>) -> impl Responder {
    logging("POST /project");
    (Json(project.0), StatusCode::CREATED)
}


#[get("/project/{id}")]
pub async fn get(id: web::Path<i32>) -> impl Responder {
    logging(&format!("GET /project/{}", id));
    format!("Project {}", id)
}
