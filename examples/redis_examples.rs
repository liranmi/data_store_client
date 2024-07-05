use data_store_client::data_store_client::DataStoreClient;
use data_store_client::store_client::interface::StoreClient;
use data_store_client::store_client::redis_client::RedisStoreClient;

#[tokio::main]
async fn main() {
    let mut redis_client = RedisStoreClient::new("redis://127.0.0.1/");
    redis_client.open_connection().await.unwrap();

    let mut data_store_client = DataStoreClient::new(redis_client);

    data_store_client
        .set_key_value("key1", "value1")
        .await
        .unwrap();
    let value = data_store_client.get_key_value("key1").await.unwrap();
    println!("Got value for key1: {:?}", value);
}
