use diesel::{PgConnection, r2d2::ConnectionManager};

use crate::exceptions::db::DatabaseException;

pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DBPooled = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub fn create_pool(database_url: &str) -> Result<DBPool, DatabaseException<'static>> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    match r2d2::Pool::builder().build(manager) {
        Ok(pool) => Ok(pool), 
        Err(_) => Err(DatabaseException::ConnexionException("Cannot create database pool !")),
    }
}

#[cfg(test)]
pub fn create_test_pool() -> DBPool {
    dotenvy::dotenv().ok();
    let url = std::env::var("DATABASE_URL_TEST").expect("Test Database url must be provided !");
    create_pool(&url.as_str())
        .expect("Cannot create test database pool !")
}