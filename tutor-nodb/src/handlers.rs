use super::state::AppState;
use actix_web::{web, HttpResponse};

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {

    let health_check_reponse = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{health_check_reponse} {visit_count} times.");
    *visit_count += 1;
    return HttpResponse::Ok().json(&response);

}