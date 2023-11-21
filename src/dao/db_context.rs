use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::{FromRow, MySqlPool};
use sqlx::mysql::MySqlRow;
use crate::model::hero::Hero;
use crate::model::log_response::Log;
use crate::model::team::Team;

pub struct Table<'c, T> where T: FromRow<'c, MySqlRow>, {
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'c MySqlRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<&'c T>,
}

impl<'c, T> Table<'c, T> where T: FromRow<'c, MySqlRow>, {
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

pub struct Database<'c> {
    pub heroes: Arc<Table<'c, Hero>>,
    pub teams: Arc<Table<'c, Team>>,
    pub logs: Arc<Table<'c, Log>>,
}

impl<'c> Database<'c> {
    pub async fn new(sql_url: &String) -> Database<'c> {
        let connection = MySqlPool::connect(&sql_url).await.unwrap();
        let pool = Arc::new(connection);

        Database {
            heroes: Arc::from(Table::new(pool.clone())),
            teams: Arc::from(Table::new(pool.clone())),
            logs: Arc::from(Table::new(pool.clone())),
        }
    }
}
