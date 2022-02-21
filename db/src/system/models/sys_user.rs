use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

use super::sys_dept::DeptResp;

static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([1]\d{10}|([\(（]?0[0-9]{2,3}[）\)]?[-]?)?([2-9][0-9]{6,7})+(\-[0-9]{1,4})?)$")
        .unwrap()
});
// static MOBILE_REGEX: Lazy<Regex> =
//     Lazy::new(||
// Regex::new(r"^1([358][0-9]|4[579]|66|7[0135678]|9[89])[0-9]{8}$").unwrap());
#[derive(Deserialize, Debug, Validate)]
pub struct AddReq {
    pub user_name: String,
    #[validate(length(min = 1))]
    pub user_nickname: Option<String>,
    pub user_password: String,
    pub user_status: Option<String>,
    #[validate(email)]
    pub user_email: String,
    pub sex: Option<String>,
    #[validate(length(min = 1))]
    pub avatar: Option<String>,
    #[validate(length(min = 1))]
    pub dept_id: String,
    #[validate(length(min = 1))]
    pub remark: Option<String>,
    pub is_admin: Option<String>,
    #[validate(regex(path = "PHONE_REGEX", code = "phone_num is invalid"))]
    pub phone_num: Option<String>,
    pub post_ids: Option<Vec<String>>,
    pub role_ids: Option<Vec<String>>,
    pub role_id: Option<String>,
}

#[derive(Deserialize, Debug, Validate)]
pub struct EditReq {
    pub id: String,
    pub user_name: String,
    pub user_nickname: String,
    pub user_status: String,
    pub user_email: String,
    pub sex: String,
    pub avatar: String,
    pub dept_id: String,
    pub remark: String,
    pub is_admin: String,
    pub phone_num: String,
    pub post_ids: Vec<String>,
    pub role_ids: Option<Vec<String>>,
    pub role_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, FromQueryResult)]
pub struct UserResp {
    pub id: String,
    pub user_name: String,
    pub user_nickname: String,
    pub user_status: String,
    pub user_email: String,
    pub sex: String,
    pub avatar: String,
    pub dept_id: String,
    pub remark: String,
    pub is_admin: String,
    pub phone_num: String,
    pub role_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct UserWithDept {
    #[serde(flatten)]
    pub user: UserResp,
    pub dept: DeptResp,
}

#[derive(Debug, Serialize, Default)]
pub struct UserInfomaion {
    pub user_info: UserResp,
    pub post_ids: Vec<String>,
    pub role_ids: Vec<String>,
    pub dept_id: String,
}

#[derive(Deserialize, Debug, Validate)]
pub struct SearchReq {
    #[validate(length(min = 1))]
    pub user_id: Option<String>,
    pub role_id: Option<String>,
    pub user_ids: Option<Vec<String>>,
    #[validate(length(min = 1))]
    pub user_name: Option<String>,
    pub phone_num: Option<String>,
    #[validate(length(min = 1))]
    pub user_nickname: Option<String>,
    pub user_status: Option<String>,
    #[validate(length(min = 1))]
    pub dept_id: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DeleteReq {
    pub user_ids: Vec<String>,
}

///  用户登录
#[derive(Deserialize, Debug, Validate)]
pub struct UserLoginReq {
    ///  用户名
    #[validate(length(min = 4, message = "用户名长度不能小于4"))]
    pub user_name: String,
    ///  用户密码
    #[validate(length(min = 6, message = "密码长度不能小于6"))]
    pub user_password: String,
    #[validate(length(min = 1, message = "验证码不能为空"))]
    pub code: String,
    pub uuid: String,
}

#[derive(Serialize, Debug)]
pub struct UserInfo {
    pub user: UserResp,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
#[derive(Deserialize)]
pub struct ResetPasswdReq {
    pub user_id: String,
    pub new_passwd: String,
}

#[derive(Deserialize, Clone)]
pub struct ChangeStatusReq {
    pub user_id: String,
    pub status: String,
}

#[derive(Deserialize, Clone)]
pub struct ChangeRoleReq {
    pub user_id: String,
    pub role_id: String,
}
