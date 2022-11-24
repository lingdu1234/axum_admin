use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct SysRoleApiAddReq {
    pub role_id: String,
    pub api: String,
    pub method: Option<String>,
}
