use std::{env, error::Error};

use async_trait::async_trait;
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

#[derive(Debug)]
struct User {
    id: i64,
    email: String,
}

#[async_trait]
trait Dao<T, ID> {
    async fn get_entity(self, id: ID) -> Result<Option<T>, Box<dyn Error + Send + Sync>>;
}
struct UserDao {
    conn_pool: Pool<Postgres>,
}

impl UserDao {
    pub fn new(conn_pool: Pool<Postgres>) -> Self {
        UserDao { conn_pool }
    }
}

#[async_trait]
impl Dao<User, i64> for UserDao {
    async fn get_entity(self, id: i64) -> Result<Option<User>, Box<dyn Error + Send + Sync>> {
        let result = sqlx::query_as!(User, "SELECT id, email FROM users WHERE id = $1", id)
            .fetch_optional(&self.conn_pool)
            .await;

        match result {
            Ok(user) => Ok(user),
            Err(e) => Err(Box::new(e)),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .unwrap();

    let user_dao = UserDao::new(pool.clone());
    match user_dao.get_entity(1).await {
        Ok(Some(user)) => println!("Found user: {:?}", user),
        Ok(None) => println!("User not found"),
        Err(e) => println!("Error occurred: {}", e),
    }

    Ok(())
}
