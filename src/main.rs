mod api;
mod config;
mod resolver;
mod services;
mod type_defs;
mod entity;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "info");
    dotenvy::dotenv().ok();
    env_logger::init();

    api::start().await
}
