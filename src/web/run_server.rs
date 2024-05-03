use crate::database::repo::Repository;
use actix_cors::Cors;
use actix_web::{dev::Server, App, HttpServer};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    config::parse_local_config,
    database::mongodb::{config::MongoDbConfig, repo_user},
    web::index,
};

use super::apidocs::ApiDoc;

pub async fn run_server() -> Option<Server> {
    let mut openapi = ApiDoc::openapi();
    openapi.info.version = env!("CARGO_PKG_VERSION").to_string();
    let config = parse_local_config("config.toml".to_string());
    let mongo_db_config = MongoDbConfig::new(config.persistence.host, config.persistence.database);
    let repo_user = repo_user::UserMongoRepo::new(&mongo_db_config).unwrap();
    repo_user.init().await.unwrap();
    println!("Starting server");
    let serve = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_header()
            .allow_any_method()
            .allow_any_origin();
        let swagger_ui =
            SwaggerUi::new("/api/docs/{_:.*}").url("/api/docs/docs.json", openapi.clone());
        App::new()
            .wrap(cors)
            .service(swagger_ui)
            .service(index::hello)
            .wrap(TracingLogger::default())
    });
    match serve.bind(("0.0.0.0", config.port)) {
        Ok(serv) => {
            // trace starting server
            Some(serv.run())
        }
        Err(err) => {
            // trace that it doesn't work
            println!("Couldn't start server {:?}", err);
            None
        }
    }
}
