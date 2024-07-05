// examples/redis_examples.rs

/// This example demonstrates how to use the `redis` crate in Rust with Tokio.
/// It covers various operations like connecting to a Redis server,
/// setting and getting values, working with lists, and using hashes in an asynchronous context.
use redis::AsyncCommands;
use tokio;

type RedisClient = redis::aio::MultiplexedConnection;
/// Establish an asynchronous connection to the Redis server.
///
/// # Arguments
///
/// * `url` - The URL of the Redis server.
///
/// # Returns
///
/// * An asynchronous Redis client connection.
///
/// # Example
///
/// ```
/// let client = connect_redis("redis://127.0.0.1/").await.unwrap();
/// ```
async fn connect_redis(url: &str) -> redis::RedisResult<RedisClient> {
    let client = redis::Client::open(url)?;
    client.get_multiplexed_async_connection().await
}

/// Set a key-value pair in Redis asynchronously.
///
/// # Arguments
///
/// * `con` - A mutable reference to the Redis connection.
/// * `key` - The key to set.
/// * `value` - The value to set.
///
/// # Example
///
/// ```
/// set_key_value(&mut con, "key1", "value1").await.unwrap();
/// ```
async fn set_key_value(con: &mut RedisClient, key: &str, value: &str) -> redis::RedisResult<()> {
    con.set(key, value).await?;
    Ok(())
}

/// Get a value by key from Redis asynchronously.
///
/// # Arguments
///
/// * `con` - A mutable reference to the Redis connection.
/// * `key` - The key to get.
///
/// # Returns
///
/// * The value associated with the key.
///
/// # Example
///
/// ```
/// let value: String = get_key_value(&mut con, "key1").await.unwrap();
/// ```
async fn get_key_value(con: &mut RedisClient, key: &str) -> redis::RedisResult<String> {
    let value: String = con.get(key).await?;
    Ok(value)
}

/// Append a value to a list in Redis asynchronously.
///
/// # Arguments
///
/// * `con` - A mutable reference to the Redis connection.
/// * `key` - The key of the list.
/// * `value` - The value to append.
///
/// # Example
///
/// ```
/// append_to_list(&mut con, "list1", "item1").await.unwrap();
/// ```
async fn append_to_list(con: &mut RedisClient, key: &str, value: &str) -> redis::RedisResult<()> {
    con.rpush(key, value).await?;
    Ok(())
}

/// Get all elements from a list in Redis asynchronously.
///
/// # Arguments
///
/// * `con` - A mutable reference to the Redis connection.
/// * `key` - The key of the list.
///
/// # Returns
///
/// * A vector of strings representing the list elements.
///
/// # Example
///
/// ```
/// let list: Vec<String> = get_list(&mut con, "list1").await.unwrap();
/// ```
async fn get_list(con: &mut RedisClient, key: &str) -> redis::RedisResult<Vec<String>> {
    let list: Vec<String> = con.lrange(key, 0, -1).await?;
    Ok(list)
}

/// Set a field in a hash in Redis asynchronously.
///
/// # Arguments
///
/// * `con` - A mutable reference to the Redis connection.
/// * `key` - The key of the hash.
/// * `field` - The field to set.
/// * `value` - The value to set.
///
/// # Example
///
/// ```
/// set_hash_field(&mut con, "hash1", "field1", "value1").await.unwrap();
/// ```
async fn set_hash_field(
    con: &mut RedisClient,
    key: &str,
    field: &str,
    value: &str,
) -> redis::RedisResult<()> {
    con.hset(key, field, value).await?;
    Ok(())
}

/// Get a field value from a hash in Redis asynchronously.
///
/// # Arguments
///
/// * `con` - A mutable reference to the Redis connection.
/// * `key` - The key of the hash.
/// * `field` - The field to get.
///
/// # Returns
///
/// * The value associated with the field.
///
/// # Example
///
/// ```
/// let value: String = get_hash_field(&mut con, "hash1", "field1").await.unwrap();
/// ```
async fn get_hash_field(
    con: &mut RedisClient,
    key: &str,
    field: &str,
) -> redis::RedisResult<String> {
    let value: String = con.hget(key, field).await?;
    Ok(value)
}

#[tokio::main]
async fn main() {
    // Connect to the Redis server.
    let mut con = connect_redis("redis://127.0.0.1/")
        .await
        .expect("Failed to connect to Redis");

    // Example 1: Set and get a key-value pair.
    set_key_value(&mut con, "key1", "value1")
        .await
        .expect("Failed to set key");
    let value: String = get_key_value(&mut con, "key1")
        .await
        .expect("Failed to get key");
    println!("Got value for key1: {}", value);

    // Example 2: Append to and get a list.
    append_to_list(&mut con, "list1", "item1")
        .await
        .expect("Failed to append to list");
    append_to_list(&mut con, "list1", "item2")
        .await
        .expect("Failed to append to list");
    let list: Vec<String> = get_list(&mut con, "list1")
        .await
        .expect("Failed to get list");
    println!("Got list1: {:?}", list);

    // Example 3: Set and get a hash field.
    set_hash_field(&mut con, "hash1", "field1", "value1")
        .await
        .expect("Failed to set hash field");
    let hash_value: String = get_hash_field(&mut con, "hash1", "field1")
        .await
        .expect("Failed to get hash field");
    println!("Got value for hash1 -> field1: {}", hash_value);
}
