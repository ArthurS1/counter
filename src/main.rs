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
    bson::doc,
    Client,
};

#[derive(Deserialize)]
struct CounterUpdate {
    modifier: i32,
}

#[derive(Serialize)]
struct CounterInfos {
    counter: i32,
}

async fn connect(addrs: &str) -> Client
{
    let client = Client::with_uri_str(addrs);

    match client.await {
        Err(_) => panic!("Failed to connect to MongoDB"),
        Ok(client) => {
            match client.database("counter").run_command(doc! {"ping": 1}, None).await {
                Err(_) => panic!("Failed pinging MongoDB"),
                Ok(_) => client,
            }
        }
    }
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
    println!("Connecting to db");
    connect("mongodb+srv://<username>:<password>@<cluster-url>/test?w=majority").await;
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
