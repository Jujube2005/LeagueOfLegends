use std::sync::Arc;

use diesel::{Connection, PgConnection, r2d2::{ConnectionManager, PooledConnection}};

use crate::{domain::repositories::transaction_provider::TransactionProvider, infrastructure::database::postgresql_connection::PgPoolSquad};

pub struct DieselTransaction {
    db_pool: Arc<PgPoolSquad>,
}

impl DieselTransaction {
    pub fn new(db_pool: Arc<PgPoolSquad>) -> Self {
        Self { db_pool }
    }
}

impl TransactionProvider for DieselTransaction {
    fn transaction<T, E, F>(&self, f: F) -> Result<T, E>
    where
        F: FnOnce(&mut PooledConnection<ConnectionManager<PgConnection>>) -> Result<T, E> + 'static,
        T: 'static,
        E: From<anyhow::Error> + From<diesel::result::Error> + 'static,
    {
        let db_pool_artifact = Arc::clone(&self.db_pool);

        let conn = &mut db_pool_artifact
            .get()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        //diesel::connection::Connection
        //fn transaction(&mut self, f: F) -> Result<T, E>
        //Executes the given function inside of a database transaction
        conn.transaction(|conn| f(conn))
    }
}
