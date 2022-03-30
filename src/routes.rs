
use crate::controllers::user_controller;
use actix_web::web;

pub fn routing(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(user_controller::home));
}