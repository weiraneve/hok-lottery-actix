mod database;
mod team_dao;
mod hero_dao;
mod log_dao;

pub type Database<'c> = database::Database<'c>;
pub type Table<'c, T> = database::Table<'c, T>;
