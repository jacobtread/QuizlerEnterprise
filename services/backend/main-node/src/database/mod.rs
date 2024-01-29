use sea_orm::DbErr;

pub mod entities;

pub type DbResult<T> = Result<T, DbErr>;
