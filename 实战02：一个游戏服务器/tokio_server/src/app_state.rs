use anyhow::Result;
use crate::redis_cache::RedisCache;
use crate::mongo_persist::MongoPersist;
use std::sync::Arc;

pub struct AppState {
    pub redis: Arc<RedisCache>,
    pub mongo: Arc<MongoPersist>,
}

impl AppState {
    pub async fn initialize() -> Result<Self> {
        // Redis 客户端
        let redis_client = redis::Client::open("redis://127.0.0.1/").expect("redis client");
        let redis = Arc::new(RedisCache::new(redis_client));

        // MongoDB 客户端
        let mut opts = mongodb::options::ClientOptions::parse("mongodb://127.0.0.1:27017").await?;
        opts.app_name = Some("TokioServer".to_string());
        let mongo_client = mongodb::Client::with_options(opts)?;
        let mongo = Arc::new(MongoPersist::new(mongo_client));

        Ok(AppState { redis, mongo })
    }
}
