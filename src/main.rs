use actix_web::{get, App, web, HttpServer, Responder, Result};
use serde::Serialize;

#[derive(Serialize)]
struct HelloWorld {
    hello_world: String,
}

#[get("/")]
async fn home() -> Result<impl Responder> 
{
    let obj = HelloWorld {
        hello_world: String::from("Hello World!"),
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    HttpServer::new(|| {
        App::new()
            .service(home)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
