use actix_web::{App, HttpServer};
use paperclip::actix::{
    OpenApiExt
};
use navigator::app_config::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .configure(config_app)
            .with_json_spec_at("/api/spec/v2")
            .build()
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}