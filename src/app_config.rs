
use paperclip::actix::web;

use crate::services::hello_world_service::{hello, echo};

pub fn config_app(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
        .service(hello)
        .service(echo));
}
