use actix_web::{get, http::StatusCode, web::Json, Responder};
use serde::Serialize;

use crate::routes::logging;

#[derive(Serialize)]
struct User {
    id: i32,
    first_name: String,
    last_name: String,
    email: String,
    age: i32,
    address: String,
    phone: String,
}

impl User {
    fn new(id: i32, first_name: String, last_name: String, email: String, age: i32, address: String, phone: String) -> Self {
        Self {
            id,
            first_name,
            last_name,
            email,
            age,
            address,
            phone,
        }
    }
}

#[get("/")]
pub async fn index() -> impl Responder {
    logging("GET /user");
    let response = User::new(
        1, 
        "John".to_string(),
        "Doe".to_string(),
        "test@example.com".to_string(),
        25,
        "123 Main St".to_string(),
        "123-456-7890".to_string()
    );
    (Json(response), StatusCode::OK)
}