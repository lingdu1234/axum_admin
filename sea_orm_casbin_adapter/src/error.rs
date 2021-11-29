use sea_orm::DbErr;
use std::{error::Error as stdError, fmt};

#[derive(Debug)]
pub enum Error {
    DbErr(DbErr),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            DbErr(db_error) => db_error.fmt(f),
        }
    }
}

impl stdError for Error {
    fn source(&self) -> Option<&(dyn stdError + 'static)> {
        use Error::*;
        match self {
            DbErr(db_err) => Some(db_err),
        }
    }
}

impl From<DbErr> for Error {
    fn from(err: DbErr) -> Self {
        Error::DbErr(err)
    }
}
