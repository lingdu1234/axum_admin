use once_cell::sync::Lazy;
use regex::Regex;
use sea_orm::FromQueryResult;
use serde::{Deserialize, Serialize};
use validator::Validate;

static PHONE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([1]\d{10}|([\(（]?0[0-9]{2,3}[）\)]?[-]?)?([2-9][0-9]{6,7})+(\-[0-9]{1,4})?)$")
        .unwrap()
});
static MOBILE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^1([358][0-9]|4[579]|66|7[0135678]|9[89])[0-9]{8}$").unwrap());
#[derive(Serialize, Deserialize, Default, Debug, Validate)]
pub struct AddReq {
    pub user_name: String,
    // #[validate(phone)]
    #[validate(regex(path = "MOBILE_REGEX", code = "mobile_phone_num is invalid"))]
    pub mobile: String,
    #[validate(length(min = 1))]
    pub user_nickname: Option<String>,
    pub birthday: Option<i32>,
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
    #[validate(length(min = 1))]
    pub address: Option<String>,
    #[validate(length(min = 1))]
    pub describe: Option<String>,
    #[validate(regex(path = "PHONE_REGEX", code = "phone_num is invalid"))]
    pub phone_num: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug, Validate)]
pub struct EditReq {
    pub user_id: String,
    pub user_name: String,
    pub mobile: String,
    pub user_nickname: String,
    pub birthday: i32,
    pub user_status: String,
    pub user_email: String,
    pub sex: String,
    pub avatar: String,
    pub dept_id: String,
    pub remark: String,
    pub is_admin: String,
    pub address: String,
    pub describe: String,
    pub phone_num: String,
    // pub post_ids: Vec<String>,
    // pub role_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, FromQueryResult)]
pub struct Resp {
    pub id: String,
    pub user_name: String,
    pub mobile: String,
    pub user_nickname: String,
    pub birthday: i32,
    pub user_status: String,
    pub user_email: String,
    pub sex: String,
    pub avatar: String,
    pub dept_id: String,
    pub remark: String,
    pub is_admin: String,
    pub address: String,
    pub describe: String,
    pub phone_num: String,
}

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct SearchReq {
    #[validate(length(min = 1))]
    pub user_id: Option<String>,
    #[validate(length(min = 1))]
    pub user_name: Option<String>,
    #[validate(length(min = 1))]
    pub user_nickname: Option<String>,
    pub user_status: Option<String>,
    #[validate(length(min = 1))]
    pub dept_id: Option<String>,
    pub begin_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct DeleteReq {
    #[validate(length(min = 1))]
    pub user_id: Option<Vec<String>>,
    #[validate(length(min = 1))]
    pub user_name: Option<Vec<String>>,
}

///  用户登录
#[derive(Deserialize, Debug, Serialize, Default, Validate)]
pub struct UserLoginReq {
    ///  用户名
    #[validate(length(min = 4))]
    pub user_name: String,
    ///  用户密码
    #[validate(length(min = 5))]
    pub user_password: String,
}

#[derive(Serialize, Debug)]
pub struct UserInfo {
    pub user: Resp,
    pub roles: Vec<String>,
    pub permissions: Vec<String>,
}
