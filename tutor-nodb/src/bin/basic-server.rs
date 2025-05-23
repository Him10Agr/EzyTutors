use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;

pub fn general_routes(cfg: &mut web::ServiceConfig) -> () {
    cfg.route("/health", web::get().to(health_check_handler));
}

pub async fn health_check_handler() -> impl Responder {
    return HttpResponse::Ok().json("Hello. EzyTutors is alove and kicking");
}

#[actix_rt::main]
async fn main() -> io::Result<()>{

    let app = move || App::new().configure(general_routes);

    HttpServer::new(app).bind("localhost:3000")?.run().await
}