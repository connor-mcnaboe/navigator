use actix_web::{HttpResponse, Responder};
use paperclip::actix::{api_v2_operation, get, post};

#[api_v2_operation]
#[get("/hello")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[api_v2_operation]
#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}
