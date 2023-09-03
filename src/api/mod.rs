use actix_web::{
    middleware::{Compress, Logger},
    web::{self, Data},
    App, HttpServer,
};

use crate::{api::graphql::graphql_route, config::config_cors, graphql::schema};

use self::playground::{graphiql_route, playground_route};

pub mod graphql;
pub mod playground;

pub async fn start() -> std::io::Result<()> {
    let server = HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema()))
            .wrap(config_cors())
            .wrap(Compress::default())
            .wrap(Logger::new("Request => %s; %a \"%r\" | time => %Dms"))
            .service(
                web::resource("/graphql")
                    .route(web::post().to(graphql_route))
                    .route(web::get().to(graphql_route)),
            )
            .service(web::resource("/playground").route(web::get().to(playground_route)))
            .service(web::resource("/graphiql").route(web::get().to(graphiql_route)))
    });
    server.bind("127.0.0.1:4000").unwrap().run().await
}
