use chrono::{Local, NaiveDateTime};
use poem::{error::BadRequest, http::StatusCode, Error, Result};

use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde_json::json;

use crate::apps::common::models::{ListData, PageParams, RespData};
use crate::utils::{
    self,
    jwt::{AuthBody, AuthPayload},
};

use super::super::entities::{prelude::SysUser, sys_user};
use super::super::models::sys_user::{AddReq, DeleteReq, EditReq, Resp, SearchReq, UserLoginReq};

/// get_user_list 获取用户列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
// #[handler]
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SearchReq,
) -> Result<ListData<Resp>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    let mut s = SysUser::find();
    // 不查找删除数据
    s = s.filter(sys_user::Column::DeletedAt.is_null());
    // 查询条件
    if let Some(x) = req.user_id {
        s = s.filter(sys_user::Column::Id.eq(x));
    }

    if let Some(x) = req.user_name {
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
    let total = s.clone().count(db).await.map_err(BadRequest)?;
    // 获取全部数据条数
    let paginator = s
        .order_by_asc(sys_user::Column::Id)
        .into_model::<Resp>()
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await.map_err(BadRequest)?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(BadRequest)?;
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
pub async fn get_by_id_or_name(db: &DatabaseConnection, search_req: SearchReq) -> Result<Resp> {
    let mut s = SysUser::find();
    // 不查找删除数据
    s = s.filter(sys_user::Column::DeletedAt.is_null());
    //
    if let Some(x) = search_req.user_id {
        s = s.filter(sys_user::Column::Id.eq(x));
    }

    if let Some(x) = search_req.user_name {
        s = s.filter(sys_user::Column::UserName.eq(x));
    }

    let result = match s.into_model::<Resp>().one(db).await.map_err(BadRequest)? {
        Some(m) => m,
        None => return Err(Error::from_string("用户不存在", StatusCode::BAD_REQUEST)),
    };

    Ok(result)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, req: AddReq) -> Result<RespData> {
    let uid = scru128::scru128().to_string();
    let salt = utils::rand_s(10);
    let passwd = utils::encrypt_password(&req.user_password, &salt);
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_user::ActiveModel {
        id: Set(uid.clone()),
        user_salt: Set(salt),
        user_name: Set(req.user_name),
        user_nickname: Set(req.user_nickname.unwrap_or_else(|| "".to_string())),
        user_password: Set(passwd),
        mobile: Set(req.mobile),
        birthday: Set(req.birthday.unwrap_or(0)),
        user_status: Set(req.user_status.unwrap_or_else(|| "1".to_string())),
        user_email: Set(req.user_email),
        sex: Set(req.sex.unwrap_or_else(|| "0".to_string())),
        dept_id: Set(req.dept_id),
        remark: Set(req.remark.unwrap_or_else(|| "".to_string())),
        is_admin: Set(req.is_admin.unwrap_or_else(|| "1".to_string())),
        address: Set(req.address.unwrap_or_else(|| "".to_string())),
        describe: Set(req.describe.unwrap_or_else(|| "".to_string())),
        phone_num: Set(req.phone_num.unwrap_or_else(|| "".to_string())),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    let txn = db.begin().await.map_err(BadRequest)?;
    let _ = SysUser::insert(user).exec(&txn).await.map_err(BadRequest)?;
    txn.commit().await.map_err(BadRequest)?;
    let res = json!({ "user_id": uid });

    Ok(RespData::with_data(res))
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<RespData> {
    let mut s = SysUser::delete_many();
    let mut flag = false;
    if let Some(x) = delete_req.user_id {
        s = s.filter(sys_user::Column::Id.is_in(x));
        flag = true;
    }

    if let Some(x) = delete_req.user_name {
        s = s.filter(sys_user::Column::UserName.is_in(x));
        flag = true;
    }
    if !flag {
        return Err(Error::from_string(
            "用户名或者用户Id必须存在一个",
            StatusCode::BAD_REQUEST,
        ));
    }

    //开始删除
    let d = s.exec(db).await.map_err(BadRequest)?;

    return match d.rows_affected {
        0 => Err(Error::from_string("用户不存在", StatusCode::BAD_REQUEST)),
        i => Ok(RespData::with_msg(&format!("成功删除{}条用户数据", i))),
    };
}

/// delete 软删除
pub async fn delete_soft(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<RespData> {
    let mut s = SysUser::update_many();
    s = s.filter(sys_user::Column::DeletedAt.is_null());
    let mut flag = false;
    if let Some(x) = delete_req.user_id {
        s = s.filter(sys_user::Column::Id.is_in(x));
        flag = true;
    }

    if let Some(x) = delete_req.user_name {
        s = s.filter(sys_user::Column::UserName.is_in(x));
        flag = true;
    }
    if !flag {
        return Err(Error::from_string(
            "用户名或者用户Id必须存在一个",
            StatusCode::BAD_REQUEST,
        ));
    }

    //开始软删除，将用户删除时间设置为当前时间
    let d = s
        .col_expr(
            sys_user::Column::DeletedAt,
            Expr::value(Local::now().naive_local() as NaiveDateTime),
        )
        .exec(db)
        .await
        .map_err(BadRequest)?;

    return match d.rows_affected {
        0 => Err(Error::from_string("用户不存在", StatusCode::BAD_REQUEST)),
        i => Ok(RespData::with_msg(&format!("成功删除{}条用户数据", i))),
    };
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: EditReq) -> Result<RespData> {
    let uid = req.user_id;
    let s_u = SysUser::find_by_id(uid.clone())
        .one(db)
        .await
        .map_err(BadRequest)?;
    let s_user: sys_user::ActiveModel = s_u.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_user::ActiveModel {
        user_name: Set(req.user_name),
        user_nickname: Set(req.user_nickname),
        mobile: Set(req.mobile),
        birthday: Set(req.birthday),
        user_status: Set(req.user_status),
        user_email: Set(req.user_email),
        sex: Set(req.sex),
        dept_id: Set(req.dept_id),
        remark: Set(req.remark),
        is_admin: Set(req.is_admin),
        address: Set(req.address),
        describe: Set(req.describe),
        phone_num: Set(req.phone_num),
        updated_at: Set(Some(now)),
        ..s_user
    };
    // 更新
    let _aa = user.update(db).await.map_err(BadRequest)?; //这个两种方式一样 都要多查询一次
                                                          // let _bb = SysUser::update(user).exec(db).await?;
                                                          //  后续更新 角色  职位等信息

    Ok(RespData::with_msg(&format!("用户<{}>数据更新成功", uid)))
}

/// 用户登录
pub async fn login(db: &DatabaseConnection, login_req: UserLoginReq) -> Result<AuthBody> {
    // 验证用户名密码不为空
    if login_req.user_name.trim().is_empty() {
        return Err(Error::from_string("用户不存在", StatusCode::BAD_REQUEST));
    }
    if login_req.user_password.trim().is_empty() {
        return Err(Error::from_string("密码不能为空", StatusCode::BAD_REQUEST));
    }
    // 根据用户名获取用户信息
    let user = match SysUser::find()
        .filter(sys_user::Column::UserName.eq(login_req.user_name.clone()))
        .one(db)
        .await
        .map_err(BadRequest)?
    {
        Some(user) => user,
        None => {
            return Err(Error::from_string("用户不存在", StatusCode::BAD_REQUEST));
        }
    };
    //  验证密码是否正确
    if utils::encrypt_password(&login_req.user_password, &user.user_salt) != user.user_password {
        return Err(Error::from_string("密码不正确", StatusCode::BAD_REQUEST));
    };
    // 注册JWT
    let claims = AuthPayload {
        id: user.id.clone(),               // 用户id
        name: login_req.user_name.clone(), // 用户名
    };

    let token = utils::authorize(claims).await.unwrap();

    Ok(token)
}
