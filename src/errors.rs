#[derive(Debug)]
pub enum ClientErrors {
    RedisError(redis::RedisError),
    OtherError(Box<dyn std::error::Error + Send + Sync>),
}

impl From<redis::RedisError> for ClientErrors {
    fn from(err: redis::RedisError) -> Self {
        ClientErrors::RedisError(err)
    }
}

impl From<Box<dyn std::error::Error + Send + Sync>> for ClientErrors {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        ClientErrors::OtherError(err)
    }
}
