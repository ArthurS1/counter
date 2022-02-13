use actix_web::{
    get,
    App,
    web,
    HttpServer,
    Responder,
    Result};
use serde::Serialize;
use serde::Deserialize;

use mongodb::{
    Client,
    options::ClientOptions,
};

#[derive(Deserialize)]
struct CounterUpdate {
    modifier: i32,
}

#[derive(Serialize)]
struct CounterInfos {
    counter: i32,
}

#[get("/")]
async fn status() -> Result<impl Responder>
{
    println!("GET /");
    let obj = CounterInfos {
        counter: 0,
    };
    Ok(web::Json(obj))
}

async fn increment(req: web::Json<CounterUpdate>) -> Result <impl Responder>
{
    println!("POST /mod");
    let obj = CounterInfos {
        counter: req.modifier,
    };
    Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    println!("Launching web server on {}:{}...", "0.0.0.0", "8080");
    println!("Try the / and /mod routes!");
    HttpServer::new(|| {
        App::new()
            .service(status)
            .route("/mod", web::post().to(increment))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
