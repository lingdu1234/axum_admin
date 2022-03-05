use sea_orm::FromQueryResult;

#[derive(Clone, Debug, FromQueryResult)]
pub struct DataScopeInfo {
    pub id: String,
    pub dept_id: String,
    pub role_id: String,
    pub data_scope: String,
}
