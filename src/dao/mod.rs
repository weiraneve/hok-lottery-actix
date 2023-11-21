mod database;
mod team_dao;
mod hero_dao;
mod log_dao;

pub type Database = database::Database;
pub type Table<T> = database::Table<T>;
