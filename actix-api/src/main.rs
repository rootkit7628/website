use actix_web::{App, HttpServer};
use routes::{project, user};

mod routes;


#[tokio::main]
async fn main() -> std::io::Result<()>{
    let hostname = "0.0.0.0";
    let port: u16 = 2000;

    let server = HttpServer::new(|| 
        App::new()
            .service(user::index)
            .service(project::index)
            .service(project::get)
            .service(project::create)
    ).bind((hostname, port))?
    .run();

    println!("Server running on port http://{}:{} ....", hostname, port);
    server.await
}
