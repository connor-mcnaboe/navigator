use actix_web::{App, HttpResponse, HttpServer, Responder};
use paperclip::actix::{
    OpenApiExt, api_v2_operation, get, post
};

#[api_v2_operation]
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[api_v2_operation]
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .service(hello)
            .service(echo)
            .with_json_spec_at("/api/spec/v2")
            .build()
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}