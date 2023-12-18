
use std::fmt::Display;

use clap::Args;
use redis::{FromRedisValue, ToRedisArgs, Client, aio::ConnectionManager};

use crate::response::ApiResult;


#[derive(Args, Debug)]
pub struct StartCommandRedisArguments {
    #[arg(help = "The location of your redis instance")]
    #[arg(env = "REDIS_URL", short = 'r', long = "redis_url")]
    #[arg(default_value = "redis://localhost")]
    pub redis_url: String,
    
}

#[derive(Clone)]
pub struct RedisPool {
    manager: ConnectionManager
}

pub async fn connect(
    StartCommandRedisArguments {
        redis_url,
        ..
    }: StartCommandRedisArguments,
) -> Result<RedisPool, anyhow::Error> {
    let client = Client::open(redis_url)?;
    let manager = ConnectionManager::new(client).await?;

    Ok(RedisPool { manager })
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
        let res = redis::cmd("GET")
            .arg(Self::format_key(namespace, key))
            .query_async::<_, Option<T>>(&mut self.manager.clone())
            .await?;

        Ok(res)
    }

    pub async fn set<T, K>(
        &self,
        namespace: &str,
        key: K,
        value: T,
        expiry: u64
    ) -> ApiResult<()>
    where
        K: Display,
        T: ToRedisArgs
    {
        redis::cmd("SET")
            .arg(Self::format_key(namespace, key))
            .arg(value)
            .arg("EX")
            .arg(expiry)
            .query_async::<_, ()>(&mut self.manager.clone())
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
        redis::cmd("DEL")
            .arg(Self::format_key(namespace, key))
            .query_async::<_, ()>(&mut self.manager.clone())
            .await?;

        Ok(())
    }

    fn format_key(namespace: &str, key: impl Display) -> String {
        format!("{namespace}:{key}")
    }
}