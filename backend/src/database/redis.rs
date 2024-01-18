
use std::fmt::Display;

use clap::Args;
use redis::{FromRedisValue, ToRedisArgs, Client, aio::ConnectionManager};

use crate::response::ApiResult;


#[derive(Args, Debug)]
pub struct RedisArguments {
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
    RedisArguments {
        redis_url,
        ..
    }: RedisArguments,
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

    pub async fn get_json<T, K>(
        &self,
        namespace: &str,
        id: K
    ) -> ApiResult<Option<T>> 
    where
        T: for <'a> serde::Deserialize<'a>,
        K: Display
    {
        let value = self.get::<String, K>(namespace, id)
            .await?
            .and_then(|v| serde_json::from_str(&v).ok());

        Ok(value)
    }

    pub async fn set<T, K>(
        &self,
        namespace: &str,
        key: K,
        value: T,
        expiry: Option<u64>
    ) -> ApiResult<()>
    where
        K: Display,
        T: ToRedisArgs
    {
        let mut cmd = redis::cmd("SET");

        cmd.arg(Self::format_key(namespace, key)).arg(value);

        if let Some(expiry) = expiry {
            cmd.arg("EX").arg(expiry);
        }

        cmd.query_async::<_, ()>(&mut self.manager.clone()).await?;

        Ok(())
    }

    pub async fn set_json<T, K>(
        &self,
        namespace: &str,
        key: K,
        data: T,
        expiry: Option<u64>
    ) -> ApiResult<()>
    where
        T: serde::Serialize,
        K: Display
    {
        let value = serde_json::to_string(&data).map_err(anyhow::Error::new)?;

        self.set(namespace, key, value, expiry).await
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