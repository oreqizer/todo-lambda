use std::env;

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;

pub struct Env {
    table_todos: String,
}

impl Env {
    pub fn new() -> Self {
        Self {
            table_todos: env::var("tableTodos").expect("Need 'tableTodos' env var"),
        }
    }

    pub fn table_todos(&self) -> &str {
        &self.table_todos
    }
}

pub struct DB {
    client: Client,
}

impl DB {
    pub async fn new() -> Self {
        let region_provider = RegionProviderChain::default_provider().or_else("eu-central-1");
        let config = aws_config::from_env().region(region_provider).load().await;

        Self {
            client: Client::new(&config),
        }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}
