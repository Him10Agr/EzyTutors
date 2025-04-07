use actix_web::{web, App, HttpServer};
use std::io;
use std::sync::Mutex;

#[path = "../handlers.rs"]
mod handlers;
#[path = "../routes.rs"]
mod routes;
#[path = "../state.rs"]
mod state;
#[path = "../models.rs"]
mod models;

use routes::*;
use state::AppState;

#[actix_rt::main]
async fn main() -> io::Result<()> {

    let shared_data = web::Data::new(AppState{
        health_check_response: String::from("I'm good. You have already asked me."),
        visit_count: Mutex::new(0),
        courses: Mutex::new(Vec::new()),
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };

    return HttpServer::new(app).bind("localhost:3000")?.run().await;
}