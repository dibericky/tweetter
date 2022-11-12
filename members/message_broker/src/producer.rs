use std::env;

use amiquip::{Connection, Publish};
use events::Event;
use anyhow::Result;
use dotenvy::dotenv;

use crate::exchange;

pub fn publish(event: &Event) -> Result<()> {
    let msg = serde_json::to_string(event).unwrap();
    let routing_key = exchange::get_routing_key(event);
    send_message(&msg, &routing_key)?;

    Ok(())
}

fn send_message(msg: &str, routing_key: &str) -> Result<()> {
    dotenv().ok();

    let rabbit_conn_url = env::var("RABBITMQ_CONNECTION").expect("RABBITMQ_CONNECTION must be set");
    // Open connection.
    let mut connection = Connection::insecure_open(&rabbit_conn_url)?;

    // Open a channel - None says let the library choose the channel ID.
    let channel = connection.open_channel(None)?;

    // Get a handle to the direct exchange on our channel.
    let exchange = exchange::declare(&channel)?;

    // Publish a message to the "hello" queue.
    exchange.publish(Publish::new(msg.as_bytes(), routing_key))?;

    connection.close()?;

    Ok(())
}