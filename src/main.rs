use actix_web::{middleware, App, HttpServer};
use paperclip::actix::{
    OpenApiExt
};
use navigator::app_config::config_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        App::new()
            .wrap_api()
            .configure(config_app)
            .with_json_spec_at("/api/spec/v2")
            .wrap(middleware::Logger::default())
            .build()
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}