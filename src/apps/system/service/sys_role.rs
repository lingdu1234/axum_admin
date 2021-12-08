use chrono::{Local, NaiveDateTime};
use poem::{
    handler,
    web::{Data, Json, Query},
    Result,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use validator::Validate;

use super::super::entities::{prelude::SysRole, sys_role};
use super::super::models::{
    sys_role::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
    PageParams,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_sort_list(
    Data(db): Data<&DatabaseConnection>,
    Query(page_params): Query<PageParams>,
    Query(search_req): Query<SearchReq>,
) -> Result<Json<serde_json::Value>> {
    //  数据验证
    match search_req.validate() {
        Ok(_) => {}
        Err(e) => return Err(e.into()),
    }

    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysRole::find();

    if let Some(x) = search_req.name {
        s = s.filter(sys_role::Column::Name.eq(x));
    }

    if let Some(x) = search_req.status {
        s = s.filter(sys_role::Column::Status.eq(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_role::Column::Id)
        .paginate(db, page_per_size);
    let num_pages = paginator.num_pages().await?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .expect("could not retrieve posts");

    Ok(Json(serde_json::json!({

            "list": list,
            "total": total,
            "total_pages": num_pages,
            "page_num": page_num,

    })))
}

pub async fn check_data_is_exist(role_name: String, db: &DatabaseConnection) -> Result<bool> {
    let s1 = SysRole::find().filter(sys_role::Column::Name.eq(role_name));

    let count1 = s1.count(db).await?;
    Ok(count1 > 0)
}

/// add 添加
#[handler]
pub async fn add(
    Data(db): Data<&DatabaseConnection>,
    Json(add_req): Json<AddReq>,
) -> Result<Json<serde_json::Value>> {
    //  数据验证
    match add_req.validate() {
        Ok(_) => {}
        Err(e) => return Err(e.into()),
    }
    //  检查字典类型是否存在
    if check_data_is_exist(add_req.clone().name, db).await? {
        return Err("信息已存在".into());
    }

    let uid = scru128::scru128();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_role::ActiveModel {
        id: Set(uid.clone()),
        name: Set(add_req.name),
        list_order: Set(add_req.list_order),
        data_scope: Set(add_req.data_scope),
        created_at: Set(Some(now)),
        status: Set(add_req.status.unwrap_or(1)),
        remark: Set(add_req.remark.unwrap_or("".to_string())),
        ..Default::default()
    };
    let txn = db.begin().await?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysRole::insert(user).exec(&txn).await?;
    txn.commit().await?;
    let resp = Json(serde_json::json!({ "id": uid }));
    Ok(resp)
}

/// delete 完全删除
#[handler]
pub async fn ddelete(
    Data(db): Data<&DatabaseConnection>,
    Json(delete_req): Json<DeleteReq>,
) -> Result<Json<serde_json::Value>> {
    let mut s = SysRole::delete_many();

    s = s.filter(sys_role::Column::Id.is_in(delete_req.role_ids));

    //开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => return Err("你要删除的数据不存在".into()),
        i => {
            return Ok(Json(serde_json::json!({
                "msg": format!("成功删除{}条数据", i)
            })))
        }
    }
}

// edit 修改
#[handler]
pub async fn edit(
    Data(db): Data<&DatabaseConnection>,
    Json(edit_req): Json<EditReq>,
) -> Result<Json<serde_json::Value>> {
    //  数据验证
    match edit_req.validate() {
        Ok(_) => {}
        Err(e) => return Err(e.into()),
    }
    //  检查字典类型是否存在
    if check_data_is_exist(edit_req.clone().name, db).await? {
        return Err("岗位信息已存在".into());
    }
    let uid = edit_req.id;
    let s_s = SysRole::find_by_id(uid.clone()).one(db).await?;
    let s_r: sys_role::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_role::ActiveModel {
        name: Set(edit_req.name),
        data_scope: Set(edit_req.data_scope),
        list_order: Set(edit_req.list_order),
        updated_at: Set(Some(now)),
        status: Set(edit_req.status),
        remark: Set(edit_req.remark),
        ..s_r
    };
    // 更新
    let _aa = act.update(db).await?; //这个两种方式一样 都要多查询一次

    return Ok(Json(serde_json::json!({
        "msg": format!("用户<{}>数据更新成功", uid)
    })));
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_by_id(
    Data(db): Data<&DatabaseConnection>,
    Query(search_req): Query<SearchReq>,
) -> Result<Json<serde_json::Value>> {
    let mut s = SysRole::find();
    //
    if let Some(x) = search_req.id {
        s = s.filter(sys_role::Column::Id.eq(x));
    } else {
        return Err("请输入id".into());
    }

    let res = match s.one(db).await? {
        Some(m) => m,
        None => return Err("该数据不存在".into()),
    };

    let result: Resp = serde_json::from_value(serde_json::json!(res))?; //这种数据转换效率不知道怎么样

    Ok(Json(serde_json::json!({ "result": result })))
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_all(Data(db): Data<&DatabaseConnection>) -> Result<Json<serde_json::Value>> {
    let s = SysRole::find()
        .filter(sys_role::Column::Status.eq(1))
        .order_by(sys_role::Column::Id, Order::Asc)
        .all(db)
        .await?;
    let result: Vec<Resp> = serde_json::from_value(serde_json::json!(s))?; //这种数据转换效率不知道怎么样
    Ok(Json(serde_json::json!({ "result": result })))
}
