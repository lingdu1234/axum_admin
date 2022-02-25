use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{prelude::SysUser, sys_user},
        models::sys_user::{
            AddReq, ChangeRoleReq, ChangeStatusReq, DeleteReq, EditReq, ResetPasswdReq, SearchReq,
            UserLoginReq, UserResp, UserWithDept,
        },
    },
};
use poem::Request;
use scru128::scru128_string;
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

use crate::utils::{
    self,
    jwt::{AuthBody, AuthPayload, Claims},
};

/// get_user_list 获取用户列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SearchReq,
) -> Result<ListData<UserWithDept>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    let mut s = SysUser::find();
    // 不查找删除数据
    s = s.filter(sys_user::Column::DeletedAt.is_null());
    // 查询条件
    if let Some(x) = req.user_id {
        s = s.filter(sys_user::Column::Id.eq(x));
    }
    if let Some(x) = req.user_ids {
        s = s.filter(sys_user::Column::Id.is_in(x));
    }

    if let Some(x) = req.user_name {
        s = s.filter(sys_user::Column::UserName.contains(&x));
    }
    if let Some(x) = req.phone_num {
        s = s.filter(sys_user::Column::UserName.contains(&x));
    }
    if let Some(x) = req.user_status {
        s = s.filter(sys_user::Column::UserStatus.eq(x));
    }
    if let Some(x) = req.dept_id {
        s = s.filter(sys_user::Column::DeptId.eq(x));
    }
    if let Some(x) = req.begin_time {
        s = s.filter(sys_user::Column::CreatedAt.gte(x));
    }
    if let Some(x) = req.end_time {
        s = s.filter(sys_user::Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 获取全部数据条数
    let paginator = s
        .order_by_asc(sys_user::Column::Id)
        .into_model::<UserResp>()
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let users = paginator.fetch_page(page_num - 1).await?;
    let mut list: Vec<UserWithDept> = Vec::new();
    for user in users {
        let dept = super::sys_dept::get_by_id(db, user.clone().dept_id).await?;
        list.push(UserWithDept { user, dept });
    }
    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

pub async fn get_un_auth_user(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SearchReq,
) -> Result<ListData<UserResp>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    let mut s = SysUser::find();
    // 不查找删除数据
    s = s.filter(sys_user::Column::DeletedAt.is_null());
    // 查询条件
    if let Some(x) = req.user_ids {
        s = s.filter(sys_user::Column::Id.is_not_in(x));
    }
    if let Some(x) = req.user_name {
        s = s.filter(sys_user::Column::UserName.contains(&x));
    }
    if let Some(x) = req.phone_num {
        s = s.filter(sys_user::Column::UserName.contains(&x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 获取全部数据条数
    let paginator = s
        .order_by_asc(sys_user::Column::Id)
        .into_model::<UserResp>()
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;
    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, user_id: &str) -> Result<UserResp> {
    let mut s = SysUser::find();
    // 不查找删除数据
    s = s.filter(sys_user::Column::DeletedAt.is_null());
    //

    s = s.filter(sys_user::Column::Id.eq(user_id));

    let result = match s.into_model::<UserResp>().one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("用户不存在")),
    };
    Ok(result)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, req: AddReq, c_user_id: String) -> Result<String> {
    let uid = scru128::scru128_string();
    let salt = utils::rand_s(10);
    let passwd = utils::encrypt_password(&req.user_password, &salt);
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_user::ActiveModel {
        id: Set(uid.clone()),
        user_salt: Set(salt),
        user_name: Set(req.user_name),
        user_nickname: Set(req.user_nickname.unwrap_or_else(|| "".to_string())),
        user_password: Set(passwd),
        user_status: Set(req.user_status.unwrap_or_else(|| "1".to_string())),
        user_email: Set(req.user_email),
        sex: Set(req.sex.unwrap_or_else(|| "0".to_string())),
        dept_id: Set(req.dept_id),
        role_id: Set(req.role_id),
        remark: Set(req.remark.unwrap_or_else(|| "".to_string())),
        is_admin: Set(req.is_admin.unwrap_or_else(|| "1".to_string())),
        phone_num: Set(req.phone_num.unwrap_or_else(|| "".to_string())),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    let txn = db.begin().await?;
    SysUser::insert(user).exec(&txn).await?;
    // 添加职位信息
    if let Some(x) = req.post_ids {
        super::sys_post::add_post_by_user_id(&txn, uid.clone(), x).await?;
    }
    // 添加角色信息
    // 先删除原有的角色信息，再添加新的角色信息
    super::sys_user_role::delete_user_role(&txn, &uid).await?;
    if let Some(x) = req.role_ids {
        super::sys_user_role::edit_user_role(&txn, &uid, x, c_user_id).await?;
    }

    txn.commit().await?;

    Ok("用户添加成功".to_string())
}

pub async fn reset_passwd(db: &DatabaseConnection, req: ResetPasswdReq) -> Result<String> {
    let salt = utils::rand_s(10);
    let passwd = utils::encrypt_password(&req.new_passwd, &salt);
    let now: NaiveDateTime = Local::now().naive_local();
    let txn = db.begin().await?;
    // 更新用户信息
    SysUser::update_many()
        .col_expr(sys_user::Column::UserPassword, Expr::value(passwd))
        .col_expr(sys_user::Column::UserSalt, Expr::value(salt))
        .col_expr(sys_user::Column::UpdatedAt, Expr::value(now))
        .filter(sys_user::Column::Id.eq(req.user_id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "密码更新成功".to_string();

    Ok(res)
}

pub async fn change_status(db: &DatabaseConnection, req: ChangeStatusReq) -> Result<String> {
    let now: NaiveDateTime = Local::now().naive_local();
    // let uid = req.user_id;
    // let s_u = SysUser::find_by_id(uid.clone())
    //     .one(db)
    //     .await
    //     ?;
    // let s_user: sys_user::ActiveModel = s_u.unwrap().into();
    // let now: NaiveDateTime = Local::now().naive_local();
    // let user = sys_user::ActiveModel {
    //     user_status: Set(req.status),
    //     updated_at: Set(Some(now)),
    //     ..s_user
    // };
    // 更新
    let txn = db.begin().await?;
    // 更新用户信息
    SysUser::update_many()
        .col_expr(
            sys_user::Column::UserStatus,
            Expr::value(req.clone().status),
        )
        .col_expr(sys_user::Column::UpdatedAt, Expr::value(now))
        .filter(sys_user::Column::Id.eq(req.user_id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "用户状态更新成功".to_string();

    Ok(res)
}

pub async fn change_role(db: &DatabaseConnection, req: ChangeRoleReq) -> Result<String> {
    let txn = db.begin().await?;
    // 更新用户信息
    SysUser::update_many()
        .col_expr(sys_user::Column::RoleId, Expr::value(req.clone().role_id))
        .filter(sys_user::Column::Id.eq(req.user_id))
        .exec(&txn)
        .await?;
    // user.update(&txn).await?;
    txn.commit().await?;
    let res = "用户角色切换成功".to_string();

    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, req: DeleteReq) -> Result<String> {
    let mut s = SysUser::delete_many();

    s = s.filter(sys_user::Column::Id.is_in(req.clone().user_ids));

    // 开始删除
    let txn = db.begin().await?;
    // 删除用户
    let d = s.exec(&txn).await?;
    // 删除用户职位数据
    super::sys_post::delete_post_by_user_id(&txn, req.user_ids.clone()).await?;
    // 删除用户角色数据
    super::sys_user_role::delete_user_role_by_user_ids(&txn, req.user_ids, None).await?;

    txn.commit().await?;
    return match d.rows_affected {
        0 => Err(anyhow!("用户不存在")),
        i => Ok(format!("成功删除{}条用户数据", i)),
    };
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: EditReq, c_user_id: String) -> Result<String> {
    let uid = req.id;
    let s_u = SysUser::find_by_id(uid.clone()).one(db).await?;
    let s_user: sys_user::ActiveModel = s_u.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_user::ActiveModel {
        user_name: Set(req.user_name),
        user_nickname: Set(req.user_nickname),
        user_status: Set(req.user_status),
        user_email: Set(req.user_email),
        sex: Set(req.sex),
        dept_id: Set(req.dept_id),
        remark: Set(req.remark),
        is_admin: Set(req.is_admin),
        phone_num: Set(req.phone_num),
        updated_at: Set(Some(now)),
        role_id: Set(req.role_id),
        ..s_user
    };
    // 更新
    let txn = db.begin().await?;
    // 更新用户信息
    user.update(&txn).await?;
    //  更新岗位信息
    // 1.先删除用户岗位关系
    super::sys_post::delete_post_by_user_id(&txn, vec![uid.clone()]).await?;
    // 2.插入用户岗位关系
    super::sys_post::add_post_by_user_id(&txn, uid.clone(), req.post_ids).await?;
    // 更新用户角色信息
    // 先删除原有的角色信息，再添加新的角色信息
    super::sys_user_role::delete_user_role(&txn, &uid).await?;
    if let Some(x) = req.role_ids {
        super::sys_user_role::edit_user_role(&txn, &uid, x, c_user_id).await?;
    }

    txn.commit().await?;
    Ok(format!("用户<{}>数据更新成功", uid))
}

/// 用户登录
pub async fn login(
    db: &DatabaseConnection,
    login_req: UserLoginReq,
    req: &Request,
) -> Result<AuthBody> {
    let mut msg = "登录成功".to_string();
    let mut status = "1".to_string();
    // 验证验证码
    if utils::encrypt_password(&login_req.code, "") != login_req.uuid {
        msg = "验证码错误".to_string();
        status = "0".to_string();
        set_login_info(
            req,
            "".to_string(),
            login_req.user_name.clone(),
            msg.clone(),
            status.clone(),
            None,
            None,
        )
        .await;
        return Err(anyhow!("验证码错误"));
    }
    // 根据用户名获取用户信息
    let user = match SysUser::find()
        .filter(sys_user::Column::UserName.eq(login_req.user_name.clone()))
        .one(db)
        .await?
    {
        Some(user) => user,
        None => {
            msg = "用户不存在".to_string();
            status = "0".to_string();
            set_login_info(
                req,
                "".to_string(),
                login_req.user_name.clone(),
                msg.clone(),
                status.clone(),
                None,
                None,
            )
            .await;
            return Err(anyhow!("用户不存在"));
        }
    };
    //  验证密码是否正确
    if utils::encrypt_password(&login_req.user_password, &user.user_salt) != user.user_password {
        msg = "密码错误".to_string();
        status = "0".to_string();
        set_login_info(
            req,
            "".to_string(),
            login_req.user_name.clone(),
            msg.clone(),
            status.clone(),
            None,
            None,
        )
        .await;
        return Err(anyhow!("密码不正确"));
    };
    // 注册JWT
    let claims = AuthPayload {
        id: user.id.clone(),               // 用户id
        name: login_req.user_name.clone(), // 用户名
    };
    let token_id = scru128_string();
    let token = utils::authorize(claims.clone(), token_id.clone())
        .await
        .unwrap();
    // 成功登录后
    //  写入登录日志

    set_login_info(
        req,
        user.id.to_string(),
        login_req.user_name.clone(),
        msg.clone(),
        status.clone(),
        Some(token_id),
        Some(token.clone()),
    )
    .await;

    Ok(token)
}

/// 用户登录
pub async fn fresh_token(user: Claims) -> Result<AuthBody> {
    // 注册JWT
    let claims = AuthPayload {
        id: user.clone().id,     // 用户id
        name: user.clone().name, // 用户名
    };
    let token = utils::authorize(claims.clone(), user.clone().token_id)
        .await
        .unwrap();
    // 成功登录后
    // 更新原始在线日志
    super::sys_user_online::update_online(user.clone().token_id, token.clone().exp).await?;

    Ok(token)
}

pub async fn set_login_info(
    req: &Request,
    u_id: String,
    user: String,
    msg: String,
    status: String,
    token_id: Option<String>,
    token: Option<AuthBody>,
) {
    let header = req.headers().to_owned();
    let remote_addr = req.remote_addr().to_owned();
    let u = utils::get_client_info(header, remote_addr).await;
    // 写入登录日志
    let u2 = u.clone();
    let status2 = status.clone();
    // 如果成功，写入在线日志
    if status == "1" {
        if let (Some(token_id), Some(token)) = (token_id, token) {
            super::sys_user_online::add(u, u_id, token_id, token.clone().exp).await;
        }
    };
    tokio::spawn(async move {
        super::sys_login_log::add(u2, user, msg, status2).await;
    });
}
