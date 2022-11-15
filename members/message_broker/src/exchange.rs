use std::env;

use amiquip::{Channel, Exchange, ExchangeDeclareOptions, ExchangeType};
use anyhow::Result;
use events::Event;

pub fn declare(channel: &Channel) -> Result<Exchange> {
    let exchange_name = env::var("RABBITMQ_EXCHANGE").unwrap_or_else(|_| "test".to_string());
    channel
        .exchange_declare(
            ExchangeType::Topic,
            &exchange_name,
            ExchangeDeclareOptions {
                durable: true,
                auto_delete: false,
                ..Default::default()
            },
        )
        .map_err(|_| anyhow::anyhow!("Unable to declare exchange"))
}

pub fn get_routing_key_prefix() -> String {
    env::var("RABBITMQ_ROUTING_KEY_PREFIX").unwrap_or_else(|_| "test".to_string())
}

pub fn get_routing_key(event: &Event) -> String {
    let prefix = get_routing_key_prefix();
    match event {
        Event::TweetAdded(_) => format!("{}.tweets.added", prefix),
        Event::TweetMessageEdited(_) => format!("{}.tweets.edited", prefix),
        Event::UserProfileAdded(_) => todo!(),
        Event::UserProfileEdited(_) => todo!(),
    }
}
