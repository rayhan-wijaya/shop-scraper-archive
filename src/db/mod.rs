use diesel::prelude::*;
use diesel::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};

enum EstablishConnectionPoolError {
    NoDatabaseUrlError,
    CreateConnectionPoolError,
}

pub fn establish_connection_pool() -> Result<Pool<ConnectionManager<MysqlConnection>>, EstablishConnectionPoolError> {
    let database_url = std::env::var("DATABASE_URL")
        .map_err(|_| EstablishConnectionPoolError::NoDatabaseUrlError)?;

    let manager = ConnectionManager::<MysqlConnection>::new(database_url);

    Pool::builder()
        .build(manager)
        .map_err(|_| EstablishConnectionPoolError::CreateConnectionPoolError)
}
