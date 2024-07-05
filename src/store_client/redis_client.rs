use async_trait::async_trait;

use redis::AsyncCommands;

use crate::errors::ClientErrors;

use super::interface::StoreClient;

// Define a struct for the Redis client.// Define a struct for the Redis client.
pub struct RedisStoreClient {
    url: String,
    connection: Option<redis::aio::MultiplexedConnection>,
}

impl RedisStoreClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            connection: None,
        }
    }
}

#[async_trait]
impl StoreClient for RedisStoreClient {
    async fn open_connection(&mut self) -> Result<(), ClientErrors> {
        let client = redis::Client::open(self.url.as_str())?;
        let res = client.get_multiplexed_async_connection().await?;
        self.connection = Some(res);
        Ok(())
    }

    async fn set_key_value(&mut self, key: &str, value: &str) -> Result<Option<()>, ClientErrors> {
        if let Some(ref mut con) = self.connection {
            con.set(key, value).await?;
        }
        Ok(Some(()))
    }

    async fn get_key_value(&mut self, key: &str) -> Result<Option<String>, ClientErrors> {
        if let Some(ref mut con) = self.connection {
            let value: Option<String> = con.get(key).await?;
            return Ok(value);
        }
        Err(ClientErrors::OtherError(
            "Connection not established".into(),
        ))
    }

    async fn delete_key(&mut self, key: &str) -> Result<Option<String>, ClientErrors> {
        if let Some(ref mut con) = self.connection {
            let value: Option<String> = con.del(key).await?;
            return Ok(value);
        }
        Err(ClientErrors::OtherError(
            "Connection not established".into(),
        ))
    }

    async fn append_to_list(&mut self, key: &str, value: &str) -> Result<(), ClientErrors> {
        if let Some(ref mut con) = self.connection {
            con.rpush(key, value).await?;
        }
        Ok(())
    }

    async fn get_list(&mut self, key: &str) -> Result<Vec<String>, ClientErrors> {
        if let Some(ref mut con) = self.connection {
            let list: Vec<String> = con.lrange(key, 0, -1).await?;
            return Ok(list);
        }
        Err(ClientErrors::OtherError(
            "Connection not established".into(),
        ))
    }
}
