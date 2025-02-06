use sqlx::{mysql::MySqlPool, Pool as MysqlPool, MySql};
use once_cell::sync::Lazy;
use std::sync::Mutex;
use utils::config::load_config;
use deadpool_redis::{Config, Pool as RedisPool, Runtime, CreatePoolError};
use tokio::sync::Mutex as TokioMutex;

static DB_POOL: Lazy<TokioMutex<Option<MysqlPool<MySql>>>> = Lazy::new(|| TokioMutex::new(None));
static REDIS_POOL: Lazy<TokioMutex<Option<RedisPool>>> = Lazy::new(|| TokioMutex::new(None));

pub async fn get_db_pool() -> Result<MysqlPool<MySql>, sqlx::Error> {
    // load config
    let config = load_config();

    let mut pool_lock = DB_POOL.lock().await; // Use async lock

    // check if the pool has already been initialized
    if let Some(pool) = &*pool_lock {
        return Ok(pool.clone());
    }

    // load database url from config file
    let database_url = config.mysql.url.clone();

    // create a new pool
    let pool = MySqlPool::connect(&database_url).await?;

    // set the pool in the static variable
    *pool_lock = Some(pool.clone());

    Ok(pool)
}

pub async fn get_redis_pool() -> Result<RedisPool, CreatePoolError> {
    // load config
    let config = load_config();
    let mut pool_lock = REDIS_POOL.lock().await; // Use async lock

    // check if the pool has already been initialized
    if let Some(pool) = &*pool_lock {
        return Ok(pool.clone());
    }

    // load redis url from config file
    let redis_url = config.redis.url.clone();

    // create a new redis client and connection manager
    let cfg = Config::from_url(redis_url);
    let pool = cfg.create_pool(Some(Runtime::Tokio1))?;

    // set the pool in the static variable
    *pool_lock = Some(pool.clone());

    Ok(pool)
}