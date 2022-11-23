use actix_web::{App, HttpServer};
use web_server::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = actix_cors::Cors::permissive();
        App::new()
            .wrap(cors)
            .service(routes::declare())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
