use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SysRoleApiAddReq {
    pub role_id: String,
    pub api: String,
    pub method: Option<String>,
}
