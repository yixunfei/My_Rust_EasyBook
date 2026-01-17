// Redis async commands with explicit types (avoid type-inference issues)
use redis::AsyncCommands;
use redis::Client as RedisClient;
use anyhow::Result;

pub struct RedisCache {
    client: RedisClient,
}

impl RedisCache {
    pub fn new(client: RedisClient) -> Self {
        Self { client }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.client.get_async_connection().await?;
        let val: Option<String> = conn.get::<&str, Option<String>>(key).await?;
        Ok(val)
    }

    pub async fn set(&self, key: &str, value: String, ttl: u64) -> Result<()> {
        let mut conn = self.client.get_async_connection().await?;
        let _ : () = conn.set::<&str, String, ()>(key, value).await?;
        if ttl > 0 {
            let _ : () = conn.expire::<&str, ()>(key, ttl as usize).await?;
        }
        Ok(())
    }
}
