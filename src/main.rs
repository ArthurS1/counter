use actix_web::{
    get,
    App,
    web,
    HttpServer,
    Responder,
    Result};
use serde::Serialize;
use serde::Deserialize;
use std::sync::Mutex;
use mongodb::{
    bson::{
        doc,
        Bson,
    },
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

    std::thread::sleep(std::time::Duration::new(5, 0));
    match client.await {
        Err(_) => panic!("Failed to connect to MongoDB"),
        Ok(client) => {
            match client
                .database("counter")
                .run_command(doc! {"ping": 1}, None)
                .await {
                Err(_) => panic!("Failed pinging MongoDB"),
                Ok(_) => client,
            }
        }
    }
}

#[get("/")]
async fn status(data: web::Data<Mutex<Client>>) -> Result<impl Responder>
{
    let counter_collection = data
        .lock()
        .unwrap()
        .database("test")
        .collection("counter");
    let document = counter_collection.find_one(doc! {}, None).await.unwrap().unwrap();
    let obj = CounterInfos {
        counter: document.get("counter").and_then(Bson::as_i32).unwrap(),
    };

    println!("GET /");
    Ok(web::Json(obj))
}

async fn increment(req: web::Json<CounterUpdate>, data: web::Data<Mutex<Client>>) -> Result <impl Responder>
{
    let counter_collection = data
        .lock()
        .unwrap()
        .database("test")
        .collection("counter");
    let document = counter_collection.find_one(doc! {}, None).await.unwrap().unwrap();
    let new_value = document.get("counter").and_then(Bson::as_i32).unwrap() + req.modifier;
    let obj = CounterInfos {
        counter: new_value,
    };

    match counter_collection.update_one(doc! {}, doc! {"counter": new_value}, None).await {
        Err(_) => panic!("Failed to update counter"),
        Ok(_) => {
            println!("POST /mod");
            Ok(web::Json(obj))
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()>
{
    let mongo_client = connect("mongodb://root:example@db").await;
    let client: web::Data<Mutex<Client>> = web::Data::new(Mutex::new(mongo_client));
    let counter_collection = client
        .lock()
        .unwrap()
        .database("test")
        .collection("counter");

    match counter_collection.insert_one(doc! {"counter": 1}, None).await {
        Err(_) => panic!("Failed to insert counter doc"),
        Ok(_) => println!("Inserted counter successfully"),
    }
    println!("Launching web server on {}:{}...", "0.0.0.0", "8080");
    println!("Try the / and /mod routes!");
    HttpServer::new(move || {
        App::new()
            .app_data(client.clone())
            .service(status)
            .route("/mod", web::post()
            .to(increment))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
