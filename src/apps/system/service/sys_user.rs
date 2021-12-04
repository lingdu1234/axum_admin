use casbin::CoreApi;
use chrono::{Local, NaiveDateTime};
use poem::{
    handler,
    web::{Data, Json, Query},
    Request, Result,
};
use sea_orm::{
    sea_query::Expr, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};

use crate::utils::{
    self,
    jwt::{AuthBody, AuthPayload},
    CasbinService,
};

use super::super::entities::prelude::*;
use super::super::entities::sys_user;
use super::super::models::sys_user::{AddReq, DeleteReq, Resp, SearchReq, UserLoginReq};
use super::super::models::PageParams;

/// get_user_list 获取用户列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Data(db): Data<&DatabaseConnection>,
    Query(page_params): Query<PageParams>,
    Query(search_req): Query<SearchReq>,
) -> Result<Json<serde_json::Value>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    let mut s = SysUser::find();
    // 不查找删除数据
    s = s.filter(sys_user::Column::DeletedAt.is_null());
    // 查询条件
    if let Some(x) = search_req.user_id {
        s = s.filter(sys_user::Column::Id.eq(x));
    }

    if let Some(x) = search_req.user_name {
        s = s.filter(sys_user::Column::UserName.eq(x));
    }
    if let Some(x) = search_req.user_status {
        s = s.filter(sys_user::Column::UserStatus.eq(x));
    }
    if let Some(x) = search_req.begin_time {
        s = s.filter(sys_user::Column::CreatedAt.gte(x));
    }
    if let Some(x) = search_req.end_time {
        s = s.filter(sys_user::Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 获取全部数据条数
    let paginator = s
        .order_by_asc(sys_user::Column::Id)
        .paginate(db, page_per_size);
    let num_pages = paginator.num_pages().await?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .expect("could not retrieve posts");
    let list_res: Vec<Resp> = serde_json::from_value(serde_json::json!(list))?; //这种数据转换效率不知道怎么样
    Ok(Json(serde_json::json!({

            "list": list_res,
            "total": total,
            "total_pages": num_pages,
            "page_num": page_num,

    })))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id_or_name(
    req: &Request,
    Data(db): Data<&DatabaseConnection>,
    Query(search_req): Query<SearchReq>,
) -> Result<Json<serde_json::Value>> {
    // let ee = req.extensions().get::<CasbinService>().unwrap();
    // let e = &ee.enforcer;

    // let e_result = e.enforce(("a", "b", "a")).unwrap();
    // println!("e_result-----------{}", e_result);

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

    let user = match s.one(db).await? {
        Some(user) => user,
        None => return Err("用户不存在".into()),
    };

    let user_res: Resp = serde_json::from_value(serde_json::json!(user))?; //这种数据转换效率不知道怎么样

    Ok(Json(serde_json::json!({ "user": user_res })))
}

/// add 添加
#[handler]
pub async fn add(
    Data(db): Data<&DatabaseConnection>,
    Json(user_add): Json<AddReq>,
) -> Result<Json<serde_json::Value>> {
    // let user = serde_json::from_value(serde_json::json!(user_add))?;
    let uid = scru128::scru128();
    let salt = utils::rand_s(10);
    let passwd = utils::encrypt_password(&user_add.user_password, &salt);
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_user::ActiveModel {
        id: Set(uid.clone()),
        user_salt: Set(salt),
        user_name: Set(user_add.user_name),
        user_nickname: Set(user_add.user_nickname.unwrap_or("".to_string())),
        user_password: Set(passwd),
        mobile: Set(user_add.mobile),
        birthday: Set(user_add.birthday.unwrap_or(0)),
        user_status: Set(user_add.user_status.unwrap_or(1)),
        user_email: Set(user_add.user_email),
        sex: Set(user_add.sex.unwrap_or(0)),
        dept_id: Set(user_add.dept_id),
        remark: Set(user_add.remark.unwrap_or("".to_string())),
        is_admin: Set(user_add.is_admin.unwrap_or(1)),
        address: Set(user_add.address.unwrap_or("".to_string())),
        describe: Set(user_add.describe.unwrap_or("".to_string())),
        phone_num: Set(user_add.phone_num.unwrap_or("".to_string())),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    user.insert(db).await?;
    let resp = Json(serde_json::json!({ "id": uid }));
    Ok(resp)
}

/// delete 完全删除
#[handler]
pub async fn ddelete(
    Data(db): Data<&DatabaseConnection>,
    Json(delete_req): Json<DeleteReq>,
) -> Result<Json<serde_json::Value>> {
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
        return Err("用户名或者用户Id必须存在一个".into());
    }

    //开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => return Err("没有你要删除的用户".into()),
        i => {
            return Ok(Json(serde_json::json!({
                "msg": format!("成功删除{}条用户数据", i)
            })))
        }
    }
}

/// delete 完全删除
#[handler]
pub async fn delete(
    Data(db): Data<&DatabaseConnection>,
    Json(delete_req): Json<DeleteReq>,
) -> Result<Json<serde_json::Value>> {
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
        return Err("用户名或者用户Id必须存在一个".into());
    }

    //开始软删除，将用户删除时间设置为当前时间
    let d = s
        .col_expr(
            sys_user::Column::DeletedAt,
            Expr::value(Local::now().naive_local() as NaiveDateTime),
        )
        .exec(db)
        .await?;

    match d.rows_affected {
        0 => return Err("没有你要删除的用户".into()),
        i => {
            return Ok(Json(serde_json::json!({
                "msg": format!("成功删除{}条用户数据", i)
            })))
        }
    }
}

/// 用户登录
#[handler]
pub async fn login(
    Data(db): Data<&DatabaseConnection>,
    Json(login_req): Json<UserLoginReq>,
) -> Result<Json<AuthBody>> {
    // 验证用户名密码不为空
    if login_req.user_name.trim().is_empty() {
        return Err("用户名不能为空".into());
    }
    if login_req.user_password.trim().is_empty() {
        return Err("密码不能为空".into());
    }
    // 根据用户名获取用户信息
    let user = match SysUser::find()
        .filter(sys_user::Column::UserName.eq(login_req.user_name.clone()))
        .one(db)
        .await?
    {
        Some(user) => user,
        None => {
            return Err("该用户不存在".into());
        }
    };
    //  验证密码是否正确
    if utils::encrypt_password(&login_req.user_password, &user.user_salt) != user.user_password {
        return Err("用户密码不正确".into());
    };
    // 注册JWT
    let claims = AuthPayload {
        id: user.id.clone(),               // 用户id
        name: login_req.user_name.clone(), // 用户名
    };

    let token = utils::authorize(Json(claims)).await.unwrap();

    Ok(token)
}
