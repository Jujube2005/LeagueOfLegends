use serde::{Deserialize, Serialize};
use diesel::QueryableByName;
use diesel::sql_types::BigInt;

#[derive(Debug, Clone, Serialize, Deserialize, QueryableByName)]
pub struct MissionSummaryModel {
    #[diesel(sql_type = BigInt)]
    pub created_count: i64,
    #[diesel(sql_type = BigInt)]
    pub joined_count: i64,
    #[diesel(sql_type = BigInt)]
    pub completed_count: i64,
    #[diesel(sql_type = BigInt)]
    pub failed_count: i64,
}
