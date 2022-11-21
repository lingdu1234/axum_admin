use std::collections::HashMap;

use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{
            prelude::{SysRole, SysRoleDept},
            sys_role, sys_role_dept, sys_user,
        },
        models::{
            sys_menu::MenuResp,
            sys_role::{AddOrCancelAuthRoleReq, DataScopeReq, SysRoleAddReq, SysRoleDeleteReq, SysRoleEditReq, SysRoleResp, SysRoleSearchReq, SysRoleStatusReq},
            sys_role_api,
        },
        prelude::SysRoleModel,
    },
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DatabaseTransaction, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
    TransactionTrait, Value,
};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SysRoleSearchReq) -> Result<ListData<SysRoleModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysRole::find();

    if let Some(x) = req.role_ids {
        if !x.is_empty() {
            let y: Vec<&str> = x.split(',').collect();
            s = s.filter(sys_role::Column::RoleId.is_in(y));
        }
    }

    if let Some(x) = req.role_id {
        if !x.is_empty() {
            s = s.filter(sys_role::Column::RoleId.eq(x));
        }
    }

    if let Some(x) = req.role_name {
        if !x.is_empty() {
            s = s.filter(sys_role::Column::RoleName.contains(&x));
        }
    }
    if let Some(x) = req.role_key {
        if !x.is_empty() {
            s = s.filter(sys_role::Column::RoleKey.contains(&x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_role::Column::Status.eq(x));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_role::Column::ListOrder)
        .order_by_asc(sys_role::Column::RoleId)
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;
    let res = ListData {
        list,
        total,
        total_pages,
        page_num,
    };
    Ok(res)
}

pub async fn check_data_is_exist(role_name: String, db: &DatabaseConnection) -> Result<bool> {
    let s1 = SysRole::find().filter(sys_role::Column::RoleName.eq(role_name));

    let count1 = s1.count(db).await?;
    Ok(count1 > 0)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, req: SysRoleAddReq, user_id: &str) -> Result<String> {
    //  检查字典类型是否存在
    if check_data_is_exist(req.clone().role_name, db).await? {
        return Err(anyhow!("数据已存在，请检查后重试"));
    }

    // 开启事务
    let txn = db.begin().await?;
    // 添加角色数据
    let role_id = self::add_role(&txn, req.clone()).await?;
    // 获取组合角色权限数据
    let role_apis = self::get_permissions_data(&txn, role_id.clone(), req.menu_ids.clone()).await?;
    // 添加角色权限数据
    super::sys_role_api::add_role_api(&txn, role_apis, user_id).await?;

    txn.commit().await?;
    Ok("添加成功".to_string())
}

// 组合角色数据
pub async fn get_permissions_data<C>(db: &C, role_id: String, menu_ids: Vec<String>) -> Result<Vec<sys_role_api::SysRoleApiAddReq>>
where
    C: TransactionTrait + ConnectionTrait,
{
    // 获取全部菜单 均为false
    let menus = super::sys_menu::get_menus(db, false, false, false).await?;
    let menu_map = menus.iter().map(|x| (x.id.clone(), x.clone())).collect::<HashMap<String, MenuResp>>();
    // 组装角色权限数据
    let mut res: Vec<sys_role_api::SysRoleApiAddReq> = Vec::new();
    for menu_id in menu_ids {
        if let Some(menu) = menu_map.get(&menu_id) {
            res.push(sys_role_api::SysRoleApiAddReq {
                role_id: role_id.clone(),
                api: menu.api.clone(),
                method: Some(menu.method.clone()),
            });
        }
    }
    Ok(res)
}

/// 添加角色数据
pub async fn add_role(txn: &DatabaseTransaction, req: SysRoleAddReq) -> Result<String> {
    let uid = scru128::new_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_role::ActiveModel {
        role_id: Set(uid.clone()),
        role_name: Set(req.role_name),
        role_key: Set(req.role_key),
        list_order: Set(req.list_order),
        data_scope: Set(req.data_scope.unwrap_or_else(|| "3".to_string())),
        created_at: Set(now),
        status: Set(req.status),
        remark: Set(req.remark),
        ..Default::default()
    };
    SysRole::insert(user).exec(txn).await?;
    Ok(uid)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: SysRoleDeleteReq) -> Result<String> {
    let txn = db.begin().await?;
    let mut s = SysRole::delete_many();
    s = s.filter(sys_role::Column::RoleId.is_in(delete_req.role_ids.clone()));
    // 开始删除
    let d = s.exec(db).await?;
    // 删除角色权限数据 和 部门权限数据
    super::sys_role_api::delete_role_api(&txn, delete_req.role_ids.clone()).await?;

    SysRoleDept::delete_many()
        .filter(sys_role_dept::Column::RoleId.is_in(delete_req.role_ids.clone()))
        .exec(&txn)
        .await?;
    // 提交事务
    txn.commit().await?;
    match d.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn eidt_check_data_is_exist(db: &DatabaseConnection, role_id: String, role_name: String, role_key: String) -> Result<bool> {
    let c1 = SysRole::find()
        .filter(sys_role::Column::RoleName.eq(role_name))
        .filter(sys_role::Column::RoleId.ne(role_id.clone()))
        .count(db)
        .await?;
    let c2 = SysRole::find()
        .filter(sys_role::Column::RoleName.eq(role_key))
        .filter(sys_role::Column::RoleId.ne(role_id.clone()))
        .count(db)
        .await?;

    Ok(c1 > 0 || c2 > 0)
}

// 编辑用户角色
pub async fn edit(db: &DatabaseConnection, req: SysRoleEditReq, created_by: &str) -> Result<String> {
    //  检查字典类型是否存在
    if eidt_check_data_is_exist(db, req.clone().role_id, req.clone().role_name, req.clone().role_key).await? {
        return Err(anyhow!("数据已存在"));
    }
    // 开启事务
    let txn = db.begin().await?;
    // 修改数据
    let uid = req.role_id;
    let s_s = SysRole::find_by_id(uid.clone()).one(&txn).await?;
    let s_r: sys_role::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_role::ActiveModel {
        role_name: Set(req.role_name),
        role_key: Set(req.role_key),
        data_scope: Set(req.data_scope),
        list_order: Set(req.list_order),
        status: Set(req.status),
        remark: Set(req.remark),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新 //这个两种方式一样 都要多查询一次
    act.update(&txn).await?;

    // 获取组合角色权限数据
    let role_apis = self::get_permissions_data(&txn, uid.clone(), req.menu_ids.clone()).await?;

    // 删除全部权限 按角色id删除
    super::sys_role_api::delete_role_api(&txn, vec![uid.clone()]).await?;

    // 添加角色权限数据
    super::sys_role_api::add_role_api(&txn, role_apis, created_by).await?;

    // 提交事务
    txn.commit().await?;

    Ok("角色数据更新成功".to_string())
}

// set_status 状态修改
pub async fn set_status(db: &DatabaseConnection, status_req: SysRoleStatusReq) -> Result<String> {
    // 开启事务
    let txn = db.begin().await?;
    // 修改数据
    let uid = status_req.role_id;
    let now: NaiveDateTime = Local::now().naive_local();
    SysRole::update_many()
        .col_expr(sys_role::Column::Status, Expr::value(status_req.status))
        .col_expr(sys_role::Column::UpdatedAt, Expr::value(now))
        .filter(sys_role::Column::RoleId.eq(uid.clone()))
        .exec(&txn)
        .await?;
    txn.commit().await?;
    let res = format!("用户<{}>状态更新成功", uid);
    Ok(res)
}

// set_status 状态修改
pub async fn set_data_scope(db: &DatabaseConnection, req: DataScopeReq) -> Result<String> {
    // 开启事务
    let txn = db.begin().await?;
    // 修改数据
    let uid = req.role_id;
    let s_s = SysRole::find_by_id(uid.clone()).one(&txn).await?;
    let s_r: sys_role::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    // 更新数据权限
    let data_scope = req.data_scope;
    let act = sys_role::ActiveModel {
        data_scope: Set(data_scope.clone()),
        updated_at: Set(Some(now)),
        ..s_r
    };
    act.update(&txn).await?;
    // 当数据权限为自定义数据时，删除全部权限，重新添加部门权限
    if data_scope == "2" {
        // 删除全部部门权限
        SysRoleDept::delete_many().filter(sys_role_dept::Column::RoleId.eq(uid.clone())).exec(&txn).await?;
        // 添加部门权限
        let mut act_datas: Vec<sys_role_dept::ActiveModel> = Vec::new();
        for dept in req.dept_ids {
            let act_data = sys_role_dept::ActiveModel {
                role_id: Set(uid.clone()),
                dept_id: Set(dept.clone()),
                created_at: Set(Some(now)),
            };
            act_datas.push(act_data);
        }
        //  批量添加部门权限
        SysRoleDept::insert_many(act_datas).exec(&txn).await?;
    }
    txn.commit().await?;
    Ok(format!("用户<{}>数据更新成功", uid))
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, req: SysRoleSearchReq) -> Result<SysRoleResp> {
    let mut s = SysRole::find();
    //
    if let Some(x) = req.role_id {
        s = s.filter(sys_role::Column::RoleId.eq(x));
    } else {
        return Err(anyhow!("id不能为空"));
    }

    let res = match s.into_model::<SysRoleResp>().one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("数据不存在")),
    };

    Ok(res)
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<SysRoleResp>> {
    let s = SysRole::find()
        .order_by_asc(sys_role::Column::ListOrder)
        .order_by_asc(sys_role::Column::RoleId)
        .into_model::<SysRoleResp>()
        .all(db)
        .await?;
    Ok(s)
}

//  获取用户角色
// pub async fn get_all_admin_role(db: &DatabaseConnection, user_id: &str) ->
// Result<Vec<String>> {     let s = SysUserRole::find()
//         .join_rev(
//             JoinType::LeftJoin,
//             sys_user::Entity::belongs_to(sys_user_role::Entity)
//                 .from(sys_user::Column::Id)
//                 .to(sys_user_role::Column::UserId)
//                 .into(),
//         )
//         // .select_with(sys_user_role::Entity)
//         .filter(sys_user::Column::Id.eq(user_id))
//         .all(db)
//         .await?;
//     let res = s.iter().map(|x| x.role_id.clone()).collect::<Vec<String>>();
//     Ok(res)
// }

pub async fn get_current_admin_role(db: &DatabaseConnection, user_id: &str) -> Result<String> {
    let user = super::sys_user::get_by_id(db, user_id).await?;
    Ok(user.user.role_id)
}

pub async fn get_auth_users_by_role_id(db: &DatabaseConnection, role_id: &str) -> Result<Vec<String>> {
    super::sys_user_role::get_user_ids_by_role_id(db, role_id).await
}

pub async fn add_role_by_user_id(db: &DatabaseConnection, user_id: &str, role_ids: Vec<String>, created_by: String) -> Result<()> {
    let txn = db.begin().await?;
    super::sys_user_role::delete_user_role(&txn, user_id).await?;
    super::sys_user_role::edit_user_role(&txn, user_id, role_ids, &created_by).await?;
    txn.commit().await?;
    Ok(())
}

pub async fn add_role_with_user_ids(db: &DatabaseConnection, user_ids: Vec<String>, role_id: String, created_by: String) -> Result<()> {
    let txn = db.begin().await?;
    super::sys_user_role::add_role_by_lot_user_ids(&txn, user_ids, role_id, &created_by).await?;
    txn.commit().await?;
    Ok(())
}

pub async fn cancel_auth_user(db: &DatabaseConnection, req: AddOrCancelAuthRoleReq) -> Result<()> {
    let txn = db.begin().await?;
    super::sys_user_role::delete_user_role_by_user_ids(&txn, req.clone().user_ids, Some(req.role_id.clone())).await?;
    // 如果用户取消了该角色授权，设置用户该角色为null
    sys_user::Entity::update_many()
        .col_expr(sys_user::Column::RoleId, Expr::value(Value::String(None)))
        .filter(sys_user::Column::Id.is_in(req.clone().user_ids))
        .filter(sys_user::Column::RoleId.eq(req.clone().role_id))
        .exec(db)
        .await?;
    txn.commit().await?;
    Ok(())
}
