mod db_context;
mod team_dao;
mod hero_dao;
mod log_dao;

pub type Database<'c> = db_context::Database<'c>;
pub type Table<'c, T> = db_context::Table<'c, T>;
