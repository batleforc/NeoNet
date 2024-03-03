extern crate neonet;
use neonet::web::apidocs::ApiDoc;
use std::fs;
use utoipa::OpenApi;

pub fn main() {
    let mut openapi = ApiDoc::openapi();
    openapi.info.version = env!("CARGO_PKG_VERSION").to_string();
    fs::write("./swagger.json", openapi.to_pretty_json().unwrap()).expect("Unable to write file");
}
