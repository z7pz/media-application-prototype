use crate::POOL;
use async_trait::async_trait;
use sqlx::postgres::{PgDatabaseError, PgRow};
use sqlx::Encode;
use sqlx::{postgres::PgArguments, Arguments, FromRow, Pool, Postgres, Row};

#[derive(Default)]
pub struct Fields {
    pub args: PgArguments,
    pub cols: Vec<String>,
}

impl<'q> Fields {
    pub fn add<
        T: 'q
            + Send
            + sqlx::Type<sqlx::Postgres>
            + sqlx::Encode<'q, sqlx::Postgres>
            + std::marker::Send,
        V: Into<String>,
    >(
        &mut self,
        arg: V,
        value: T,
    ) {
        self.args.add(value);
        self.cols.push(arg.into());
    }
}

use std::format as f;

#[async_trait]
pub trait Base
where
    for<'a> Self: Sized + FromRow<'a, PgRow> + Unpin,
{
    fn fields(&self) -> Fields;
    fn table_name() -> &'static str;

    fn pool() -> &'static Pool<Postgres> {
        POOL.get().unwrap()
    }

    async fn insert(&self) -> Result<(), sqlx::error::Error> {
        let fields = self.fields();
        let cols = fields.cols;
        let args = fields.args;

        let args_placeholders = (1..cols.len() + 1)
            .map(|c| f!("${c}"))
            .collect::<Vec<String>>();

        let query = f!(
            "INSERT INTO {} ({}) VALUES ({})",
            Self::table_name(),
            cols.join(","),
            args_placeholders.join(",")
        );
        sqlx::query_with(&query, args).execute(Self::pool()).await?;
        Ok(())
    }
    //find_all, find, findone, findlast
    async fn find<
        'a,
        T: Into<String> + Send,
        V: Encode<'a, Postgres> + Send + sqlx::Type<sqlx::Postgres> + 'a,
    >(
        filter: T,
        vec: Vec<V>,
    ) -> anyhow::Result<Vec<Self>> {
        let filter: String = filter.into();
        let query = f!("SELECT * FROM {} WHERE {}", Self::table_name(), filter);
        let mut args = PgArguments::default();
        for arg in vec {
            args.add(arg);
        }
        let data = sqlx::query_as_with::<_, Self, _>(query.as_str(), args)
            .fetch_all(Self::pool())
            .await?;
        Ok(data)
    }

    async fn find_all() -> anyhow::Result<Vec<Self>> {
        let query = f!("SELECT * FROM {}", Self::table_name());
        let data = sqlx::query_as::<_, Self>(query.as_str())
            .fetch_all(Self::pool())
            .await?;
        Ok(data)
    }
    async fn find_one<
        'a,
        T: Into<String> + Send,
        V: Encode<'a, Postgres> + Send + sqlx::Type<sqlx::Postgres> + 'a,
    >(
        filter: T,
        vec: Vec<V>,
    ) -> Result<Self, sqlx::Error> {
        let filter: String = filter.into();
        let query = f!(
            "SELECT * FROM {} WHERE {} LIMIT 1",
            Self::table_name(),
            filter
        );

        let mut args = PgArguments::default();
        for arg in vec {
            args.add(arg);
        }
        let data = sqlx::query_as_with::<_, Self, _>(query.as_str(), args)
            .fetch_one(Self::pool())
            .await?;
        Ok(data)
    }
}
