use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

// Configure router
pub fn general_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/health", web::get().to(health_check_handler));
}

// Configure handler
pub async fn health_check_handler() -> impl Responder {
    HttpResponse::Ok().json("Actix Web Service is running!")
}

// Instantiate HTTP server and top
#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Build app and configure router
    let app = move || App::new().configure(general_routes);

    // Run HTTP server
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}