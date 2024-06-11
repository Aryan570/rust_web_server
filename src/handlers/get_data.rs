use actix_web::{get, web, HttpResponse, Responder};
use mongodb::{bson::{doc, Document}, Client};
use crate::models::get_data::GetData;

#[get("/data/{key}")]
async fn get_data(key: web::Path<String>, mongo_client: web::Data<Client>) -> impl Responder {
    let database = mongo_client.database("test");
    let collection: mongodb::Collection<Document> = database.collection("key_val");

    let filter = doc! { "key": key.as_str() };
    match collection.find_one(filter, None).await {
        Ok(Some(document)) => {
            let value: String = document.get_str("value").unwrap().to_string();
            let data = GetData {
                key: key.to_string(),
                value,
            };
            HttpResponse::Ok().json(data)
        },
        Ok(None) => HttpResponse::NotFound().body("Key not found in MongoDB"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to fetch data from MongoDB"),
    }
}
