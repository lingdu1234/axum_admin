/*
 * @Author: lingdu waong2005@126.com
 * @Date: 2022-07-17 20:10:06
 * @LastEditors: lingdu waong2005@126.com
 * @LastEditTime: 2022-10-24 09:46:28
 * @FilePath: \axum_admin\db\src\system\models\sys_dept.rs
 * @Description: 这是默认设置,请设置`customMade`, 打开koroFileHeader查看配置 进行设置: https://github.com/OBKoro1/koro1FileHeader/wiki/%E9%85%8D%E7%BD%AE
 */
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use chrono::NaiveDateTime;

#[derive(Deserialize, Debug)]
pub struct SearchReq {
    pub dept_id: Option<String>,
    pub dept_name: Option<String>,
    pub status: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AddReq {
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
}

#[derive(Deserialize)]
pub struct DeleteReq {
    pub dept_id: String,
}

#[derive(Deserialize, Clone, Debug)]
pub struct EditReq {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, FromQueryResult, Default, Deserialize)]
pub struct DeptResp {
    pub dept_id: String,
    pub parent_id: String,
    pub dept_name: String,
    pub order_num: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub created_at: NaiveDateTime,
    pub status: String,
}

#[derive(Serialize, Clone, Debug, Default)]
pub struct RespTree {
    #[serde(flatten)]
    pub data: DeptResp,
    pub children: Option<Vec<RespTree>>,
}
