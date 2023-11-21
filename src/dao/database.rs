use std::marker::PhantomData;
use std::sync::Arc;
use sqlx::{FromRow, MySqlPool};
use sqlx::mysql::MySqlRow;
use crate::model::hero::Hero;
use crate::model::log_response::Log;
use crate::model::team::Team;

pub struct Table<T> where T: FromRow<'static, MySqlRow>, {
    pub pool: Arc<MySqlPool>,
    _from_row: fn(&'static MySqlRow) -> Result<T, sqlx::Error>,
    _marker: PhantomData<T>,
}

impl<T> Table<T> where T: FromRow<'static, MySqlRow>, {
    fn new(pool: Arc<MySqlPool>) -> Self {
        Table {
            pool,
            _from_row: T::from_row,
            _marker: PhantomData,
        }
    }
}

pub struct Database {
    pub heroes: Arc<Table<Hero>>,
    pub teams: Arc<Table<Team>>,
    pub logs: Arc<Table<Log>>,
}

impl Database {
    pub async fn new(sql_url: &String) -> Self {
        let connection = MySqlPool::connect(&sql_url).await.unwrap();
        let pool = Arc::new(connection);

        Database {
            heroes: Arc::from(Table::new(pool.clone())),
            teams: Arc::from(Table::new(pool.clone())),
            logs: Arc::from(Table::new(pool.clone())),
        }
    }
}
