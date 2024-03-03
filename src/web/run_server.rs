use actix_cors::Cors;
use actix_web::{dev::Server, App, HttpServer};
use tracing_actix_web::TracingLogger;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::web::index;

use super::apidocs::ApiDoc;

pub async fn run_server() -> Option<Server> {
    let mut openapi = ApiDoc::openapi();
    openapi.info.version = env!("CARGO_PKG_VERSION").to_string();
    let port: u16 = match env!("PORT").parse() {
        Ok(port) => port,
        Err(err) => {
            println!("Couldn't parse port, starting with 16667 : {:?}", err);
            16667
        }
    };
    println!("Starting server");
    match HttpServer::new(move || {
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
    })
    .bind(("0.0.0.0", port))
    {
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
