use actix_web::{web, Scope};

pub fn init_api() -> Scope {
    web::scope("/api")
}
