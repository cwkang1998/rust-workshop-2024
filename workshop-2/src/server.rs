use std::{clone, collections::HashMap, sync::Mutex};

use actix_web::{
    error::ErrorInternalServerError, get, post, web, App, Error, HttpRequest, HttpResponse,
    HttpServer, Responder, Result,
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct KVData {
    key: String,
    value: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[get("/data")]
async fn get_data(db: web::Data<Mutex<HashMap<String, String>>>) -> Result<impl Responder> {
    let data_collection = db
        .lock()
        .unwrap()
        .clone()
        .iter()
        .map(|(k, v)| KVData {
            key: k.clone().to_string(),
            value: v.clone().to_string(),
        })
        .collect::<Vec<KVData>>();
    Ok(HttpResponse::Ok().json(data_collection))
}

#[post("/data")]
async fn post_data(
    db: web::Data<Mutex<HashMap<String, String>>>,
    data: web::Json<KVData>,
) -> Result<impl Responder> {
    db.lock()
        .unwrap()
        .insert(data.key.clone(), data.value.clone());

    Ok(HttpResponse::Ok().json(data.0))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        let key_value_store: HashMap<String, String> = HashMap::new();

        App::new()
            .app_data(web::Data::new(Mutex::new(key_value_store.clone())))
            .service(get_data)
            .service(post_data)
            .service(hello)
    })
    .bind(("127.0.0.1", 9090))?
    .run()
    .await
}
