use std::env;

use amiquip::{Connection, ConsumerMessage, ConsumerOptions, FieldTable, QueueDeclareOptions};
use anyhow::Result;
use dotenvy::dotenv;
use events::Event;

use crate::exchange::{self, get_routing_key_prefix};

pub fn consume<F>(queue: &str, handler: F) -> Result<()>
where
    F: Fn(&Event) -> Result<()>,
{
    dotenv().ok();

    let rabbit_conn_url = env::var("RABBITMQ_CONNECTION").expect("RABBITMQ_CONNECTION must be set");

    // Open connection.
    let mut connection = Connection::insecure_open(&rabbit_conn_url)?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    let queue = channel.queue_declare(queue, QueueDeclareOptions::default())?;
    let exchange = exchange::declare(&channel)?;
    queue.bind(
        &exchange,
        format!("{}.#", get_routing_key_prefix()),
        FieldTable::new(),
    )?;
    // Start a consumer.
    let consumer = queue.consume(ConsumerOptions::default())?;
    println!("Waiting for messages. Press Ctrl-C to exit.");

    for (_, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(delivery) => {
                let body = String::from_utf8_lossy(&delivery.body).to_string();
                let event = serde_json::from_str::<Event>(&body);
                match event {
                    Ok(event) => {
                        println!("Received: {:?}", &body);
                        handler(&event)?;
                        consumer.ack(delivery)?;
                    }
                    Err(_) => {
                        println!("Failed to parse message {}", &body);
                        consumer.nack(delivery, false)?;
                    }
                };
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()?;
    Ok(())
}
