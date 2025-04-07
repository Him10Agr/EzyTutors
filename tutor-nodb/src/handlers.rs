use super::state::AppState;
use super::models::Course;
use actix_web::{web, HttpResponse};
use chrono::Utc;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {

    let health_check_reponse = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{health_check_reponse} {visit_count} times.");
    *visit_count += 1;
    return HttpResponse::Ok().json(&response);

}

pub async fn new_courses(new_course: web::Json<Course>, app_state: web::Data<AppState>) -> HttpResponse {

    println!("Recieved the new course");
    let course_count_for_user = app_state.courses.lock().unwrap().clone().into_iter()
                                                    .filter(|course| course.tutor_id == new_course.tutor_id)
                                                    .count();
    
    let new_course = Course {
        tutor_id: new_course.tutor_id,
        posted_time: Some(Utc::now().naive_utc()),
        course_name: new_course.course_name.clone(),
        course_id: Some({
            let this: Result<i32, std::num::TryFromIntError> = (course_count_for_user + 1).try_into();
            match this {
                Ok(t) => t,
                Err(_e) => panic!("Failed to convert usize into u32"),
            }
        }),
    };

    app_state.courses.lock().unwrap().push(new_course);
    return HttpResponse::Ok().json("Added course");

}

pub async fn get_courses_for_tutor(app_state: web::Data<AppState>, params: web::Path<i32>) -> HttpResponse {

    let tutor_id = params.into_inner();
    let courses = app_state.courses.lock().unwrap().clone().into_iter()
                                    .filter(|course| course.tutor_id == tutor_id)
                                    .collect::<Vec<Course>>();
    
    if courses.len() > 0 {
        return HttpResponse::Ok().json(courses);
    } else {
        return HttpResponse::Ok().json("No courses found for the tutor".to_string());
    }

}

pub async fn get_course_details(app_state: web::Data<AppState>, params: web::Path<(i32, i32)>) -> HttpResponse {
    let params = params.into_inner();
    let tutor_id = params.0;
    let course_id = params.1;

    let courses = app_state.courses.lock().unwrap().clone().into_iter()
                                    .filter(|course| (course.tutor_id == tutor_id) && (course.course_id.unwrap() == course_id))
                                    .collect::<Vec<Course>>();
    
    if courses.len() > 0 {
        return HttpResponse::Ok().json(courses);
    } else {
        let json = HttpResponse::Ok().json("No course found with given course id".to_string());
        return json;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::http::StatusCode;
    use std::sync::Mutex;

    #[actix_rt::test]
    async fn post_course_test() {

        let course = web::Json(Course{
            tutor_id: 1,
            course_name: "Hello this is test course".to_string(),
            course_id: None,
            posted_time: None,
        });

        let app_state: web::Data<AppState> = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(Vec::new()),
        });

        let response = new_courses(course, app_state).await;

        assert_eq!(response.status(), StatusCode::OK);

    }

    #[actix_web::test]
    async fn get_all_courses_success() {

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(Vec::new()),
        });

        let tutor_id = web::Path::from(1);
        let response = get_courses_for_tutor(app_state, tutor_id).await;

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn get_one_course_success() {

        let app_state = web::Data::new(AppState {
            health_check_response: "".to_string(),
            visit_count: Mutex::new(0),
            courses: Mutex::new(Vec::new()),
        });

        let params = web::Path::from((1, 1));
        let response = get_course_details(app_state, params).await;
        assert_eq!(response.status(), StatusCode::OK);
    }
}