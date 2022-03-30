
use actix_web::web;
use crate::controllers::user_controller;

pub fn routing(cfg: &mut web::ServiceConfig) { 
    cfg.route("/", web::get().to(user_controller::home));
}