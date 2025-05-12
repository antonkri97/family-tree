use actix_web::web;

use super::{
    auth::{get_me_handler, login_user_handler, logout_handler, register_user_handler},
    common::health_checker_handler,
    oauth::google_oauth_handler,
};

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(register_user_handler)
        .service(login_user_handler)
        .service(google_oauth_handler)
        .service(logout_handler)
        .service(get_me_handler);

    conf.service(scope);
}
