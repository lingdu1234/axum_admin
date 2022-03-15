use anyhow::Result;
use configs::CFG;
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::models::sys_user::{
        AddReq, ChangeRoleReq, ChangeStatusReq, DeleteReq, EditReq, ResetPwdReq, SearchReq, UpdateProfileReq, UpdatePwdReq, UserInfo, UserInfomaion, UserLoginReq, UserWithDept,
    },
    DB,
};
use poem::{
    handler,
    web::{Json, Multipart, Query},
    Request,
};
use validator::Validate;

use super::super::service;
use crate::utils::jwt::{AuthBody, Claims};

/// get_user_list 获取用户列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SearchReq>) -> Res<ListData<UserWithDept>> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户

#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Res<UserInfomaion> {
    match req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    };
    match req.user_id {
        Some(user_id) => match self::get_user_info_by_id(&user_id).await {
            Err(e) => Res::with_err(&e.to_string()),
            Ok(res) => Res::with_data(res),
        },
        None => Res::with_msg("用户id不能为空"),
    }
}

#[handler]
pub async fn get_profile(user: Claims) -> Res<UserInfomaion> {
    match self::get_user_info_by_id(&user.id).await {
        Err(e) => Res::with_err(&e.to_string()),
        Ok(res) => Res::with_data(res),
    }
}

pub async fn get_user_info_by_id(id: &str) -> Result<UserInfomaion> {
    let db = DB.get_or_init(db_conn).await;

    match service::sys_user::get_by_id(db, id).await {
        Err(e) => Err(e),
        Ok(user) => {
            let post_ids = service::sys_post::get_post_ids_by_user_id(db, &user.user.id).await.unwrap();
            let role_ids = service::sys_user_role::get_role_ids_by_user_id(db, &user.user.id).await.expect("角色id获取失败");
            let res = UserInfomaion {
                user_info: user.clone(),
                dept_id: user.user.dept_id,
                post_ids,
                role_ids,
            };
            Ok(res)
        }
    }
}

/// add 添加
#[handler]
pub async fn add(Json(add_req): Json<AddReq>, user: Claims) -> Res<String> {
    match add_req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    }
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::add(db, add_req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::delete(db, delete_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>, user: Claims) -> Res<String> {
    match edit_req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    }
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::edit(db, edit_req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn update_profile(Json(req): Json<UpdateProfileReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::update_profile(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// 用户登录
#[handler]
pub async fn login(Json(login_req): Json<UserLoginReq>, request: &Request) -> Res<AuthBody> {
    match login_req.validate() {
        Ok(_) => {}
        Err(e) => return Res::with_err(&e.to_string()),
    }
    let db = DB.get_or_init(db_conn).await;
    match service::sys_user::login(db, login_req, request).await {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
/// 获取用户登录信息
#[handler]
pub async fn get_info(user: Claims) -> Res<UserInfo> {
    let db = DB.get_or_init(db_conn).await;
    //  获取用户信息
    let user_info = match service::sys_user::get_by_id(db, &user.id).await {
        Ok(x) => x,
        Err(e) => return Res::with_err(&e.to_string()),
    };

    let role_id = match user_info.user.role_id.clone() {
        Some(x) => x,
        None => "".to_string(),
    };

    let role_ids = match service::sys_role::get_all_admin_role(db, &user.id).await {
        Ok(x) => x,
        Err(e) => return Res::with_err(&e.to_string()),
    };

    // 检查是否超管用户
    let permissions = if CFG.system.super_user.contains(&user.id) {
        vec!["*:*:*".to_string()]
    } else {
        match service::sys_menu::get_role_permissions(db, vec![role_id]).await {
            Ok((x, _)) => x,
            Err(e) => return Res::with_err(&e.to_string()),
        }
    };
    // 获取用户菜单信息
    let res = UserInfo {
        user: user_info,
        roles: role_ids,
        permissions,
    };

    Res::with_data(res)
}

// edit 修改
#[handler]
pub async fn reset_passwd(Json(req): Json<ResetPwdReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::reset_passwd(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn update_passwd(Json(req): Json<UpdatePwdReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::update_passwd(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn change_status(Json(req): Json<ChangeStatusReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::change_status(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
// fresh_token 刷新token
#[handler]
pub async fn fresh_token(user: Claims) -> Res<AuthBody> {
    let res = service::sys_user::fresh_token(user).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn change_role(Json(req): Json<ChangeRoleReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_user::change_role(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn update_avatar(multipart: Multipart, user: Claims) -> Res<String> {
    let res = service::common::upload_file(multipart).await;
    match res {
        Ok(x) => {
            let db = DB.get_or_init(db_conn).await;
            let res = service::sys_user::update_avatar(db, &x, &user.id).await;
            match res {
                Ok(y) => Res::with_data_msg(x, &y),
                Err(e) => Res::with_err(&e.to_string()),
            }
        }
        Err(e) => Res::with_err(&e.to_string()),
    }
}
