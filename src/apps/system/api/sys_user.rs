use poem::{
    handler,
    web::{Json, Query},
    Request, Result,
};

use validator::Validate;

use crate::{
    apps::common::models::{PageParams, Res},
    CFG,
};
use crate::{
    apps::system::{
        models::sys_user::{UserInfo, UserWithDept},
        service,
    },
    utils::jwt::AuthBody,
};
use crate::{
    apps::{common::models::ListData, system::models::sys_user::UserInfomaion},
    utils::jwt::Claims,
};
use crate::{db_conn, DB};

use super::super::models::sys_user::{
    AddReq, ChangeStatusReq, DeleteReq, EditReq, ResetPasswdReq, SearchReq, UserLoginReq,
};

/// get_user_list 获取用户列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Query(page_params): Query<PageParams>,
    Query(req): Query<SearchReq>,
) -> Json<Res<ListData<UserWithDept>>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Json(Res::with_data(x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Json<Res<UserInfomaion>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    };
    let db = DB.get_or_init(db_conn).await;
    match req.user_id {
        Some(user_id) => match service::sys_user::get_by_id(db, user_id).await {
            Err(e) => Json(Res::with_err(&e.to_string())),
            Ok(user) => {
                let post_ids = service::sys_post::get_post_ids_by_user_id(db, user.clone().id)
                    .await
                    .unwrap();
                let role_ids = service::sys_role::get_role_ids_by_user_id(&user.clone().id).await;
                let res = UserInfomaion {
                    user_info: user.clone(),
                    dept_id: user.dept_id,
                    post_ids,
                    role_ids,
                };
                Json(Res::with_data(res))
            }
        },
        None => Json(Res::with_msg("用户id不能为空")),
    }
}

/// add 添加
#[handler]
pub async fn add(Json(add_req): Json<AddReq>) -> Json<Res<String>> {
    match add_req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    }
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::add(db, add_req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Json<Res<String>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::delete(db, delete_req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>) -> Json<Res<String>> {
    match edit_req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    }
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::edit(db, edit_req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

/// 用户登录
#[handler]
pub async fn login(Json(login_req): Json<UserLoginReq>, request: &Request) -> Json<Res<AuthBody>> {
    match login_req.validate() {
        Ok(_) => {}
        Err(e) => return Json(Res::with_err(&e.to_string())),
    }
    let db = DB.get_or_init(db_conn).await;
    match service::sys_user::login(db, login_req, request).await {
        Ok(res) => Json(Res::with_data(res)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
/// 获取用户登录信息
#[handler]
pub async fn get_info(user: Claims) -> Result<Json<Res<UserInfo>>> {
    let db = DB.get_or_init(db_conn).await;
    //  获取用户信息
    let user_info = service::sys_user::get_by_id(db, user.id.clone()).await?;
    //    获取角色列表
    let all_roles = service::sys_role::get_all(db).await?;
    //  获取 用户角色
    let roles = service::sys_role::get_admin_role(&user.id, all_roles).await?;
    // let mut role_names: Vec<String> = Vec::new();
    let mut role_ids: Vec<String> = Vec::new();

    for role in roles {
        // role_names.push(role.name);
        role_ids.push(role.role_id);
    }

    // 检查是否超管用户
    let permissions = if CFG.system.super_user.contains(&user.id) {
        vec!["*:*:*".to_string()]
    } else {
        service::sys_menu::get_permissions(role_ids.clone()).await
    };
    // let permissions = service::sys_menu::get_permissions(role_ids.clone()).await;
    // 获取用户菜单信息
    let res = UserInfo {
        user: user_info,
        roles: if role_ids.is_empty() {
            vec!["".to_string()]
        } else {
            role_ids
        },
        permissions,
    };

    Ok(Json(Res::with_data(res)))
}

// edit 修改
#[handler]
pub async fn reset_passwd(Json(req): Json<ResetPasswdReq>) -> Json<Res<String>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::reset_passwd(db, req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}

// edit 修改
#[handler]
pub async fn change_status(Json(req): Json<ChangeStatusReq>) -> Json<Res<String>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::change_status(db, req).await;
    match res {
        Ok(x) => Json(Res::with_msg(&x)),
        Err(e) => Json(Res::with_err(&e.to_string())),
    }
}
// fresh_token 刷新token
#[handler]
pub async fn fresh_token(user: Claims) -> Result<Json<Res<AuthBody>>> {
    let res = service::sys_user::fresh_token(user).await;
    match res {
        Ok(x) => Ok(Json(Res::with_data(x))),
        Err(e) => Ok(Json(Res::with_err(&e.to_string()))),
    }
}
