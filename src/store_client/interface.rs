use crate::errors::ClientErrors;
use async_trait::async_trait;

#[async_trait]
pub trait StoreClient: Send + Sync {
    async fn open_connection(&mut self) -> Result<(), ClientErrors>;
    async fn set_key_value(&mut self, key: &str, value: &str) -> Result<Option<()>, ClientErrors>;
    async fn get_key_value(&mut self, key: &str) -> Result<Option<String>, ClientErrors>;
    async fn delete_key(&mut self, key: &str) -> Result<Option<String>, ClientErrors>;
    async fn append_to_list(&mut self, key: &str, value: &str) -> Result<(), ClientErrors>;
    async fn get_list(&mut self, key: &str) -> Result<Vec<String>, ClientErrors>;
}
