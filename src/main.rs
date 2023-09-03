mod api;
mod config;
mod graphql;
mod services;
mod schemas;
mod entity;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "info");
    dotenvy::dotenv().ok();
    env_logger::init();

    api::start().await
}
