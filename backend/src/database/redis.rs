
use std::fmt::Display;

use clap::Args;
use redis::{FromRedisValue, ToRedisArgs, aio::ConnectionManager, Client};

use crate::response::ApiResult;


#[derive(Args, Debug)]
pub struct StartCommandRedisArguments {
    #[arg(help = "The location of your redis instance")]
    #[arg(env = "REDIS_URL", short = 'r', long = "redis_url")]
    #[arg(default_value = "redis://localhost")]
    pub redis_url: String,
    
}

#[derive(Clone)]
pub struct RedisPool(pub ConnectionManager);

pub fn connect(
    StartCommandRedisArguments {
        redis_url,
        ..
    }: StartCommandRedisArguments,
) -> Result<RedisPool, anyhow::Error> {
    let client = Client::open(redis_url)?;

    Ok(RedisPool(ConnectionManager::new(client)))
}

impl RedisPool {
    pub async fn get<T, K>(
        &self,
        namespace: &str,
        key: K
    ) -> ApiResult<Option<T>> 
    where 
        K: Display,
        T: FromRedisValue
    {
        let mut conn = self.0.get().await?;

        let res = redis::cmd("GET")
            .arg(Self::format_key(namespace, key))
            .query_async::<_, Option<T>>(&mut conn)
            .await?;

        Ok(res)
    }

    pub async fn set<T, K>(
        &self,
        namespace: &str,
        key: K,
        value: T,
        expiry: i64
    ) -> ApiResult<()>
    where
        K: Display,
        T: ToRedisArgs
    {
        let mut conn = self.0.get().await?;

        redis::cmd("SET")
            .arg(Self::format_key(namespace, key))
            .arg(value)
            .arg("EX")
            .arg(expiry)
            .query_async::<_, ()>(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn delete<K>(
        &self,
        namespace: &str,
        key: K
    ) -> ApiResult<()>
    where
        K: Display
    {
        let mut conn = self.0.get().await?;

        redis::cmd("DEL")
            .arg(Self::format_key(namespace, key))
            .query_async::<_, ()>(&mut conn)
            .await?;

        Ok(())
    }

    fn format_key(namespace: &str, key: impl Display) -> String {
        format!("{namespace}:{key}")
    }
}