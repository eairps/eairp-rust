use sqlx::{mysql::MySqlPool, Pool, MySql};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use utils::config::load_config;

static DB_POOL: Lazy<Mutex<Option<Pool<MySql>>>> = Lazy::new(|| Mutex::new(None));

pub async fn get_db_pool() -> Result<Pool<MySql>, sqlx::Error> {
    // load config
    let config = load_config();
    let mut pool_lock = DB_POOL.lock().unwrap();

    // check if the pool has already been initialized
    if let Some(pool) = &*pool_lock {
        return Ok(pool.clone());
    }

    // load database url from config file
    let database_url = config.database.url.clone();

    // create a new pool
    let pool = MySqlPool::connect(&database_url).await?;

    // set the pool in the static variable
    *pool_lock = Some(pool.clone());

    Ok(pool)
}
