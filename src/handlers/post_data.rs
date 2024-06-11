use actix_web::{post, web, HttpResponse, Responder};
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;
use crate::models::post_data::PostData;

#[post("/data")]
async fn post_data(data: web::Json<PostData>, kafka_producer: web::Data<FutureProducer>) -> impl Responder {
    let record = FutureRecord::to("soup")
        .key(&data.key)
        .payload(&data.value);
    match kafka_producer.send(record, Duration::from_secs(0)).await {
        Ok(_) => HttpResponse::Ok().body("Data sent to Kafka"),
        Err(_) => HttpResponse::InternalServerError().body("Failed to send data to Kafka"),
    }
}
