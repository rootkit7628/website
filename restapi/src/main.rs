use axum::{
    routing::{delete, get, post, put},
    Router,
};
use dotenv::dotenv;
use routes::project::ProjectRoutes;
use std::{env, net::SocketAddr};

pub mod database;
pub mod models;
pub mod routes;

#[tokio::main]
async fn main() {
    dotenv().ok();
    match database::init() {
        Ok(_) => {
            let hostname: String = env::var("HOSTNAME").unwrap_or("0.0.0.0".to_string());
            let port = env::var("PORT")
                .unwrap_or("5000".to_string())
                .parse::<u16>()
                .unwrap();

            let app = Router::new()
                .route("/project", get(ProjectRoutes::get_all))
                .route("/project/:id", get(ProjectRoutes::get))
                .route("/project", post(ProjectRoutes::create))
                .route("/project/:id", put(ProjectRoutes::update))
                .route("/project/:id", delete(ProjectRoutes::delete));

            let listener = tokio::net::TcpListener::bind(format!("{}:{}", hostname, port))
                .await
                .unwrap();
            println!("Server running on [ http://{}:{} ]...", hostname, port);
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .unwrap();
        }
        Err(e) => {
            eprintln!("Error: {}", e);
        }
    }
}
