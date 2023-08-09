use actix_web::{middleware, App, HttpServer, http};
use navigator::app_config::config_app;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://localhost:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .configure(config_app)
            .wrap(middleware::Logger::default())
            .wrap(cors )
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}