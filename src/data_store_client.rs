// Define a trait that will be implemented by different clients.

use crate::{errors::ClientErrors, store_client::interface::StoreClient};

pub struct DataStoreClient<T: StoreClient> {
    client: T,
}

impl<T: StoreClient> DataStoreClient<T> {
    pub fn new(client: T) -> Self {
        Self { client }
    }
    // Example of a method that uses the client.
    pub async fn set_key_value(
        &mut self,
        key: &str,
        value: &str,
    ) -> Result<Option<()>, ClientErrors> {
        self.client.set_key_value(key, value).await
    }

    pub async fn get_key_value(&mut self, key: &str) -> Result<Option<String>, ClientErrors> {
        self.client.get_key_value(key).await
    }
}
