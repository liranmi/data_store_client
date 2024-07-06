use std::collections::HashMap;

use async_trait::async_trait;

use crate::{errors::ClientErrors, store_client::interface::StoreClient};

#[allow(dead_code)]
pub struct MockStoreClient {
    url: String,
    connection: Option<HashMap<String, String>>,
}

impl MockStoreClient {
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            connection: None,
        }
    }
}

#[async_trait]
impl StoreClient for MockStoreClient {
    async fn open_connection(&mut self) -> Result<(), ClientErrors> {
        self.connection = Some(HashMap::new());
        Ok(())
    }
    async fn set_key_value(&mut self, key: &str, value: &str) -> Result<Option<()>, ClientErrors> {
        if let Some(ref mut con) = self.connection {
            con.insert(key.to_string(), value.to_string());
            return Ok(None);
        }
        return Err(ClientErrors::OtherError("Store is not available".into()));
    }
    async fn get_key_value(&mut self, key: &str) -> Result<Option<String>, ClientErrors> {
        if let Some(ref mut con) = self.connection {
            return Ok(con.get(key).map(|val| val.to_string()));
        }
        return Err(ClientErrors::OtherError("Store is not available".into()));
    }
    async fn delete_key(&mut self, key: &str) -> Result<Option<String>, ClientErrors> {
        if let Some(ref mut con) = self.connection {
            return Ok(con.remove(key).map(|val| val.to_string()));
        }
        return Err(ClientErrors::OtherError("Store is not available".into()));
    }
    async fn append_to_list(&mut self, _key: &str, _value: &str) -> Result<(), ClientErrors> {
        todo!()
    }
    async fn get_list(&mut self, _key: &str) -> Result<Vec<String>, ClientErrors> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_open_connection() {
        let mut client = MockStoreClient::new("http://localhost");
        let result = client.open_connection().await;
        assert!(result.is_ok());
        assert!(client.connection.is_some());
    }

    #[tokio::test]
    async fn test_set_key_value() {
        let mut client = MockStoreClient::new("http://localhost");
        client.open_connection().await.unwrap();

        let result = client.set_key_value("key1", "value1").await;
        assert!(result.is_ok());
        assert_eq!(
            client.connection.unwrap().get("key1"),
            Some(&"value1".to_string())
        );
    }

    #[tokio::test]
    async fn test_get_key_value() {
        let mut client = MockStoreClient::new("http://localhost");
        client.open_connection().await.unwrap();
        client.set_key_value("key1", "value1").await.unwrap();

        let result = client.get_key_value("key1").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_delete_key() {
        let mut client = MockStoreClient::new("http://localhost");
        client.open_connection().await.unwrap();
        client.set_key_value("key1", "value1").await.unwrap();

        let result = client.delete_key("key1").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Some("value1".to_string()));
    }

    #[tokio::test]
    async fn test_set_key_value_store_not_available() {
        let mut client = MockStoreClient::new("http://localhost");

        let result = client.set_key_value("key1", "value1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_get_key_value_store_not_available() {
        let mut client = MockStoreClient::new("http://localhost");

        let result = client.get_key_value("key1").await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_delete_key_store_not_available() {
        let mut client = MockStoreClient::new("http://localhost");

        let result = client.delete_key("key1").await;
        assert!(result.is_err());
    }
}
