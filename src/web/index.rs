use actix_web::get;

#[get("/")]
pub async fn hello() -> &'static str {
    "Hello to NeoNet!"
}
