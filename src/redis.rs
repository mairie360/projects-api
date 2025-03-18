use r2d2::{Pool, PooledConnection};
use r2d2_redis::RedisConnectionManager;

#[derive(Clone)]
pub struct Redis {
    pool: Pool<RedisConnectionManager>,
}

impl Redis {
    pub fn new(redis_url: &str) -> Self {
        let manager = RedisConnectionManager::new(redis_url).unwrap();
        let pool = Pool::builder().build(manager).unwrap();
        Redis { pool }
    }

    pub fn get_connection(&self) -> PooledConnection<RedisConnectionManager> {
        self.pool
            .get()
            .expect("Failed to get a connection from the pool.")
    }
}