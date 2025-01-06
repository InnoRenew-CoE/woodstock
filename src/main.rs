use actix_web::{get, web, App, HttpServer, Responder};
use actix_web_lab::web::spa;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(
            spa()
                .index_file("public/index.html")
                .static_resources_location("public/")
                .finish(),
        )
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}

// TODO: Static Files:      https://github.com/actix/examples/blob/master/basics/static-files/src/main.rs
// TODO: Nested routing:    https://github.com/actix/examples/tree/master/basics/nested-routing
// TODO: Postgres usage:    https://github.com/actix/examples/tree/master/databases/postgres/src
