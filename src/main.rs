mod handlers;
mod services;
mod models;

use actix_web::{web, App, HttpServer};
use rdkafka::config::ClientConfig;
use rdkafka::producer::FutureProducer;
use mongodb::{options::ClientOptions, Client as MongoClient};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Kafka configuration
    let kafka_producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Producer creation error");

    // MongoDB configuration
    let mongo_client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
    let mongo_client = MongoClient::with_options(mongo_client_options).unwrap();
    let mongo_client_clone = mongo_client.clone();

    // Spawn a background task for Kafka to MongoDB processing
    tokio::spawn(async move {
        services::kafka_to_mongo::process_kafka_to_mongo(mongo_client_clone).await;
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(kafka_producer.clone()))
            .app_data(web::Data::new(mongo_client.clone()))
            .configure(handlers::init_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
