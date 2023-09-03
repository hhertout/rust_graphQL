mod api;
mod config;
mod graphql;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();

    api::start().await
}
