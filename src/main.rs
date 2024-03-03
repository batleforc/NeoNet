use neonet::web::run_server::run_server;

extern crate neonet;
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let _ = match run_server().await {
        Some(serv) => serv.await,
        None => return Ok(()),
    };
    Ok(())
}
