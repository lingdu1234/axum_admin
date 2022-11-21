use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Clone,ToSchema)]
pub struct AddEditReq {
    pub api_id: String,
    pub dbs: Vec<String>,
}

#[derive(Deserialize,ToSchema)]
pub struct SearchReq {
    pub api_id: String,
}
