use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use std::sync::Arc;

#[derive(Clone)]
pub struct Postgres {
    pool: Arc<Pool<ConnectionManager<PgConnection>>>,
}

impl Postgres {
    pub fn new(postgres_url: &str) -> Self {
        let manager = ConnectionManager::<PgConnection>::new(postgres_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");

        Self {
            pool: Arc::new(pool),
        }
    }

    pub fn get_connection(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        self.pool
            .get()
            .expect("Failed to get a connection from the pool.")
    }
}
