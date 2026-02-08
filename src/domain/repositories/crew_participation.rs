use anyhow::Result;
use async_trait::async_trait;
use diesel::{
    PgConnection,
    r2d2::{ConnectionManager, PooledConnection},
};
use mockall::automock;

use crate::domain::entities::crew_memberships::CrewMemberShips;

#[async_trait]
#[automock]
pub trait CrewOperationRepository {
    async fn join(&self, crew_memberships: CrewMemberShips) -> Result<()>;
    async fn leave(&self, crew_memberships: CrewMemberShips) -> Result<()>;

    //testing method
    fn for_insert_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMemberShips,
    ) -> Result<()>;
    fn for_delete_transaction_test(
        &self,
        conn: &mut PooledConnection<ConnectionManager<PgConnection>>,
        crew_memberships: CrewMemberShips,
    ) -> Result<()>;

}