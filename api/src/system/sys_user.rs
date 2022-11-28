use app_service::{
    service_utils::jwt::{AuthBody, Claims},
    system,
};
use axum::{
    extract::{Multipart, Query},
    Json,
};
use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::models::sys_user::{
        ChangeDeptReq, ChangeRoleReq, ChangeStatusReq, ResetPwdReq, SysUserAddReq, SysUserDeleteReq, SysUserEditReq, SysUserSearchReq, UpdateProfileReq, UpdatePwdReq, UserInfo,
        UserInformation, UserLoginReq, UserWithDept,
    },
    DB,
};
use headers::HeaderMap;
use tokio::join;

#[utoipa::path(
    get,
    path = "/system/user/list",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取用户列表", body = UserWithDept),
    ),
    params(
        ("page_params" = PageParams, Query, description = "分页参数"),
        ("params" = SysUserSearchReq, Query, description = "查询参数"),
    ),
)]
/// 获取用户列表
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SysUserSearchReq>) -> Res<ListData<UserWithDept>> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/user/get_by_id",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "按id获取用户信息", body = UserInformation)
    ),
    params(
        ("params" = SysPostSearchReq, Query, description = "查询参数")
    ),
)]
/// 按id获取用户信息
pub async fn get_by_id(Query(req): Query<SysUserSearchReq>) -> Res<UserInformation> {
    let db = DB.get_or_init(db_conn).await;
    match req.user_id {
        Some(user_id) => match system::sys_user::get_user_info_by_id(db, &user_id).await {
            Err(e) => Res::with_err(&e.to_string()),
            Ok(res) => Res::with_data(res),
        },
        None => Res::with_msg("用户id不能为空"),
    }
}

#[utoipa::path(
    get,
    path = "/system/user/get_profile",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取用户个人信息", body = UserInformation)
    )
)]
/// 获取用户个人信息
pub async fn get_profile(user: Claims) -> Res<UserInformation> {
    let db = DB.get_or_init(db_conn).await;
    match system::sys_user::get_user_info_by_id(db, &user.id).await {
        Err(e) => Res::with_err(&e.to_string()),
        Ok(res) => Res::with_data(res),
    }
}

/// add 添加

#[utoipa::path(
    post,
    path = "/system/user/add",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "新增用户", body = String)
    ),
    request_body = SysUserAddReq,
)]
/// 新增用户
pub async fn add(user: Claims, Json(add_req): Json<SysUserAddReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::add(db, add_req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    delete,
    path = "/system/user/delete",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "删除用户", body = String)
    ),
    request_body = SysUserDeleteReq,
)]
/// 删除用户
pub async fn delete(Json(delete_req): Json<SysUserDeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::delete(db, delete_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/user/edit",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新用户", body = String)
    ),
    request_body = SysUserEditReq,
)]
/// 更新用户
pub async fn edit(user: Claims, Json(edit_req): Json<SysUserEditReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::edit(db, edit_req, user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/user/update_profile",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新用户个人信息", body = String)
    ),
    request_body = UpdateProfileReq,
)]
/// 更新用户个人信息
pub async fn update_profile(Json(req): Json<UpdateProfileReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::update_profile(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    post,
    path = "/comm/login",
    tag = "common",
    responses(
        (status = 200, description = "用户登录", body = AuthBody)
    ),
    request_body = UserLoginReq,
)]
/// 用户登录
pub async fn login(header: HeaderMap, Json(login_req): Json<UserLoginReq>) -> Res<AuthBody> {
    let db = DB.get_or_init(db_conn).await;
    match system::sys_user::login(db, login_req, header).await {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    get,
    path = "/system/user/get_info",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "获取用户完全信息", body = UserInfo),
    )
)]
/// 获取用户完全信息
pub async fn get_info(user: Claims) -> Res<UserInfo> {
    let db = DB.get_or_init(db_conn).await;

    let (role_ids_r, dept_ids_r, user_r) = join!(
        system::sys_user_role::get_role_ids_by_user_id(db, &user.id),
        system::sys_user_dept::get_dept_ids_by_user_id(db, &user.id),
        system::sys_user::get_user_info_permission(db, &user.id),
    );

    let roles = match role_ids_r {
        Ok(x) => x,
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let depts = match dept_ids_r {
        Ok(x) => x,
        Err(e) => return Res::with_err(&e.to_string()),
    };
    let (user, permissions) = match user_r {
        Ok((x, y)) => (x, y),
        Err(e) => return Res::with_err(&e.to_string()),
    };

    let res = UserInfo { user, roles, depts, permissions };

    Res::with_data(res)
}

// edit 修改

#[utoipa::path(
    put,
    path = "/system/user/reset_passwd",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "重置密码", body = String)
    ),
    request_body = ResetPwdReq,
)]
/// 重置密码
pub async fn reset_passwd(Json(req): Json<ResetPwdReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::reset_passwd(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/user/update_passwd",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新密码", body = String)
    ),
    request_body = UpdatePwdReq,
)]
/// 更新密码
pub async fn update_passwd(user: Claims, Json(req): Json<UpdatePwdReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::update_passwd(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/user/change_status",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新用户状态", body = String)
    ),
    request_body = ChangeStatusReq,
)]
/// 更新用户状态
pub async fn change_status(Json(req): Json<ChangeStatusReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_status(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/user/fresh_token",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新用户状态", body = AuthBody)
    ),
)]
/// 刷新token
pub async fn fresh_token(user: Claims) -> Res<AuthBody> {
    let res = system::sys_user::fresh_token(user).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/user/change_role",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "改变用户角色", body = String)
    ),
    request_body = ChangeRoleReq,
)]
/// 改变用户角色
pub async fn change_role(Json(req): Json<ChangeRoleReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_role(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/user/change_dept",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "改变用户部门", body = String)
    ),
    request_body = ChangeRoleReq,
)]
/// 改变用户部门
pub async fn change_dept(Json(req): Json<ChangeDeptReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = system::sys_user::change_dept(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[utoipa::path(
    put,
    path = "/system/user/update_avatar",
    tag = "SysUser",
    security(("authorization" = [])),
    responses(
        (status = 200, description = "更新头像", body = String)
    ),
    request_body = Multipart,
)]
/// 更新头像
pub async fn update_avatar(user: Claims, multipart: Multipart) -> Res<String> {
    let res = system::common::upload_file(multipart).await;
    match res {
        Ok(x) => {
            let db = DB.get_or_init(db_conn).await;
            let res = system::sys_user::update_avatar(db, &x, &user.id).await;
            match res {
                Ok(y) => Res::with_data_msg(x, &y),
                Err(e) => Res::with_err(&e.to_string()),
            }
        }
        Err(e) => Res::with_err(&e.to_string()),
    }
}
