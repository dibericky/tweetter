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
    match event {
        Event::UserTweetAdded(_) => routing_key("user.tweets.added"),
        Event::UserTweetMessageEdited(_) => routing_key("user.tweets.edited"),
        Event::UserCreated(_) => routing_key("user.created"),
        Event::UserEdited(_) => routing_key("user.edited"),
        Event::UserNumberTweetIncremented(_) => routing_key("user.details.num_tweets.incremented"),
    }
}

fn routing_key(name: &str) -> String {
    let prefix = get_routing_key_prefix();
    format!("{prefix}.{name}")
}
