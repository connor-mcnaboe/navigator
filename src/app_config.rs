use actix_web::web;

use crate::controllers::mission_controller::{mission};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mission")
        .service(mission));
}
