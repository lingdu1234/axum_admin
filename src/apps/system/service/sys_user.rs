use poem::{
    handler,
    web::{Data, Json, Query},
    IntoResponse, Result,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};

use super::super::entities::prelude::*;
use super::super::entities::sys_user;
use super::super::models::sys_user::{UserAddReq, UserResp, UserSearchReq};
use super::super::models::PageParams;

/// get_user_list 获取用户列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_user_list(
    db: Data<&DatabaseConnection>,
    Query(page_params): Query<PageParams>,
    Query(user_search_req): Query<UserSearchReq>,
) -> Result<Json<serde_json::Value>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    let mut s = SysUser::find();

    if let Some(x) = user_search_req.user_id {
        s = s.filter(sys_user::Column::Id.eq(x));
    }

    if let Some(x) = user_search_req.user_name {
        s = s.filter(sys_user::Column::UserName.eq(x));
    }

    let paginator = s
        .order_by_asc(sys_user::Column::Id)
        .paginate(db.0, page_per_size);
    let num_pages = paginator.num_pages().await?;
    let users = paginator
        .fetch_page(page_num - 1)
        .await
        .expect("could not retrieve posts");

    Ok(Json(serde_json::json!({
        "code": 0,
        "msg": "success",
        "data": {
            "list": users,
            "total": num_pages,
            "page_num": page_num,
        }
    })))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_user_by_id(
    db: Data<&DatabaseConnection>,
    user_search_req: Query<UserSearchReq>,
) -> Result<Json<serde_json::Value>> {
    let user_id = match user_search_req.user_id.to_owned() {
        Some(user_id) => user_id,
        None => Err("user_id is empty")?,
    };
    let user = match SysUser::find_by_id(user_id).one(db.0).await? {
        Some(user) => user,
        None => return Err("用户不存在".into()),
    };

    let user_res: UserResp = serde_json::from_value(serde_json::json!(user))?; //这种数据转换效率不知道怎么样

    Ok(Json(serde_json::json!({
        "code": 0,
        "msg": "success",
        "data": user_res
    })))
}

/// add_user 添加用户
#[handler]
pub async fn add_user(
    Data(db): Data<&DatabaseConnection>,
    Json(user_add): Json<UserAddReq>,
) -> Result<Json<serde_json::Value>> {
    // let user = serde_json::from_value(serde_json::json!(user_add))?;
    let uid = scru128::scru128();
    let user = sys_user::ActiveModel {
        id: Set(uid.clone()),
        user_salt: Set("dfsdf".to_owned()),
        user_name: Set(user_add.user_name),
        mobile: Set(user_add.mobile),
        ..Default::default()
    };

    user.insert(db).await?;
    let resp = Json(serde_json::json!({
        "code": 0,
        "msg": "success",
        "data": {
            "id":uid
        },
    }));
    Ok(resp)
}
