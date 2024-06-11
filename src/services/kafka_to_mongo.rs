use rdkafka::config::ClientConfig;
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::message::Message;
use mongodb::{Client, bson::doc};
use futures::stream::StreamExt;
// use futures_util::stream::stream::StreamExt;
pub async fn process_kafka_to_mongo(mongo_client: Client) {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", "my-group")
        .set("bootstrap.servers", "localhost:9092")
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&["soup"]).expect("Failed to subscribe to topic");

    let database = mongo_client.database("test");
    let collection = database.collection("key_val");

    while let Some(message) = consumer.stream().next().await {
        match message {
            Ok(m) => {
                if let Some(payload) = m.payload() {
                    let key = match m.key() {
                        Some(key) => std::str::from_utf8(key).unwrap_or(""),
                        None => "",
                    };
                    println!("{}",key);
                    let value = std::str::from_utf8(payload).unwrap();
                    let doc = doc!{ "key": key, "value": value };
                    match collection.insert_one(doc, None).await {
                        Ok(_) => println!("Inserted document into MongoDB"),
                        Err(e) => eprintln!("Failed to insert document: {:?}", e),
                    }
                }
            }
            Err(e) => eprintln!("Kafka error: {:?}", e),
        }
    }
}
