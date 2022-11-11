use std::env;

use amiquip::{Connection, ExchangeType, Publish, ExchangeDeclareOptions};
use events::Event;
use anyhow::Result;
use dotenvy::dotenv;

fn get_routing_key (event: &Event) -> String {
    let prefix = env::var("RABBITMQ_ROUTING_KEY_PREFIX").unwrap_or_else(|_| "test".to_string());
    match event {
        Event::TweetAdded(_) => format!("{}.tweets.added", prefix),
        Event::TweetMessageEdited(_) => format!("{}.tweets.edited", prefix),
    }
}

pub fn publish(event: &Event) -> Result<()> {
    let msg = serde_json::to_string(event).unwrap();
    let routing_key = get_routing_key(event);
    send_message(&msg, &routing_key)?;

    Ok(())
}

fn send_message(msg: &str, routing_key: &str) -> Result<()> {
    dotenv().ok();

    let database_url = env::var("RABBITMQ_CONNECTION").expect("DATABASE_URL must be set");
    let exchange_name = env::var("RABBITMQ_EXCHANGE").unwrap_or_else(|_| "test".to_string());
    // Open connection.
    let mut connection = Connection::insecure_open(&database_url)?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = channel.exchange_declare(ExchangeType::Direct, &exchange_name, ExchangeDeclareOptions {
        durable: true,
        auto_delete: false,
        ..Default::default()
    })?;

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(msg.as_bytes(), routing_key))?;

    connection.close()?;

    Ok(())
}