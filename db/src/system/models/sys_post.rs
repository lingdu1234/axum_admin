use sea_orm::{entity::prelude::DateTime, FromQueryResult};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Debug, ToSchema)]
pub struct SysPostSearchReq {
    pub post_id: Option<String>,
    pub post_code: Option<String>,
    pub post_name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug, ToSchema)]
pub struct SysPostAddReq {
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Deserialize, ToSchema)]
pub struct SysPostDeleteReq {
    pub post_ids: Vec<String>,
}

#[derive(Deserialize, Clone, Debug, ToSchema)]
pub struct SysPostEditReq {
    pub post_id: String,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: Option<String>,
}

#[derive(Debug, Serialize, FromQueryResult, ToSchema)]
pub struct SysPostResp {
    pub post_id: String,
    pub post_code: String,
    pub post_name: String,
    pub post_sort: i32,
    pub status: String,
    pub remark: String,
    pub created_at: DateTime,
}
