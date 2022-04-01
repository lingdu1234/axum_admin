use db::{
    common::res::{ListData, PageParams, Res},
    db_conn,
    system::{
        entities::sys_role,
        models::{
            sys_role::{AddOrCancelAuthRoleReq, AddReq, DataScopeReq, DeleteReq, EditReq, Resp, SearchReq, StatusReq, UpdateAuthRoleReq},
            sys_user::{SearchReq as UserSearchReq, UserResp, UserWithDept},
        },
    },
    DB,
};
use poem::{
    handler,
    web::{Json, Query},
};

use super::super::service;
use crate::utils::jwt::Claims;

/// get_list 获取列表
/// page_params 分页参数
#[handler]
pub async fn get_sort_list(Query(page_params): Query<PageParams>, Query(req): Query<SearchReq>) -> Res<ListData<sys_role::Model>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// add 添加
#[handler]
pub async fn add(Json(req): Json<AddReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::add(db, req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(Json(delete_req): Json<DeleteReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::delete(db, delete_req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn edit(Json(edit_req): Json<EditReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::edit(db, edit_req, &user.id).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// set_status 修改状态
#[handler]
pub async fn change_status(Json(req): Json<StatusReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::set_status(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
// set_data_scope 修改数据权限范围
#[handler]
pub async fn set_data_scope(Json(req): Json<DataScopeReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::set_data_scope(db, req).await;
    match res {
        Ok(x) => Res::with_msg(&x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_user_by_id 获取用户Id获取用户
#[handler]
pub async fn get_by_id(Query(req): Query<SearchReq>) -> Res<Resp> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_by_id(db, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_all 获取全部
#[handler]
pub async fn get_all() -> Res<Vec<Resp>> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::get_all(db).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

/// get_role_menu 获取角色授权菜单id数组
#[handler]
pub async fn get_role_menu(Query(req): Query<SearchReq>) -> Res<Vec<String>> {
    let db = DB.get_or_init(db_conn).await;
    match req.role_id {
        None => Res::with_msg("role_id不能为空"),
        Some(id) => {
            let api_ids = match service::sys_menu::get_role_permissions(db, &id).await {
                Ok((_, x)) => x,
                Err(e) => return Res::with_err(&e.to_string()),
            };
            Res::with_data(api_ids)
        }
    }
}

/// get_role_dept 获取角色授权部门id数组
#[handler]
pub async fn get_role_dept(Query(req): Query<SearchReq>) -> Res<Vec<String>> {
    match req.role_id {
        None => Res::with_msg("role_id不能为空"),
        Some(id) => {
            let db = DB.get_or_init(db_conn).await;
            let res = service::sys_dept::get_dept_by_role_id(db, id).await;
            match res {
                Ok(x) => Res::with_data(x),
                Err(e) => Res::with_err(&e.to_string()),
            }
        }
    }
}

#[handler]
pub async fn get_auth_users_by_role_id(Query(mut req): Query<UserSearchReq>, Query(page_params): Query<PageParams>) -> Res<ListData<UserWithDept>> {
    let db = DB.get_or_init(db_conn).await;
    let role_id = match req.role_id.clone() {
        None => return Res::with_err("角色Id不能为空"),
        Some(id) => id,
    };
    let user_ids = match service::sys_role::get_auth_users_by_role_id(db, &role_id).await {
        Ok(x) => x,
        Err(e) => return Res::with_err(&e.to_string()),
    };
    req.user_ids = Some(user_ids);
    let res = service::sys_user::get_sort_list(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn get_un_auth_users_by_role_id(Query(mut req): Query<UserSearchReq>, Query(page_params): Query<PageParams>) -> Res<ListData<UserResp>> {
    let db = DB.get_or_init(db_conn).await;
    let role_id = match req.role_id.clone() {
        None => return Res::with_err("角色Id不能为空"),
        Some(id) => id,
    };
    let user_ids = match service::sys_role::get_auth_users_by_role_id(db, &role_id).await {
        Ok(x) => x,
        Err(e) => return Res::with_err(&e.to_string()),
    };
    req.user_ids = Some(user_ids);
    let res = service::sys_user::get_un_auth_user(db, page_params, req).await;
    match res {
        Ok(x) => Res::with_data(x),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

// edit 修改
#[handler]
pub async fn update_auth_role(Json(req): Json<UpdateAuthRoleReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    match service::sys_role::add_role_by_user_id(db, &req.user_id, req.role_ids, user.id).await {
        Ok(_) => Res::with_msg("角色授权更新成功"),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn add_auth_user(Json(req): Json<AddOrCancelAuthRoleReq>, user: Claims) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::add_role_with_user_ids(db, req.clone().user_ids, req.role_id, user.id).await;
    match res {
        Ok(_) => Res::with_msg("授权成功"),
        Err(e) => Res::with_err(&e.to_string()),
    }
}

#[handler]
pub async fn cancel_auth_user(Json(req): Json<AddOrCancelAuthRoleReq>) -> Res<String> {
    let db = DB.get_or_init(db_conn).await;
    let res = service::sys_role::cancel_auth_user(db, req).await;
    match res {
        Ok(_) => Res::with_msg("取消授权成功"),
        Err(e) => Res::with_err(&e.to_string()),
    }
}
