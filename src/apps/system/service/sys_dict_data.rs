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

use super::super::entities::{
    prelude::SysDictData,
    sys_dict_data::{ActiveModel, Column},
};
use super::super::models::{
    sys_dict_data::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
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
    let mut s = SysDictData::find();

    if let Some(x) = search_req.dict_type {
        s = s.filter(Column::DictType.eq(x));
    }

    if let Some(x) = search_req.dict_label {
        s = s.filter(Column::DictLabel.eq(x));
    }
    if let Some(x) = search_req.status {
        s = s.filter(Column::Status.eq(x));
    }
    if let Some(x) = search_req.begin_time {
        s = s.filter(Column::CreatedAt.gte(x));
    }
    if let Some(x) = search_req.end_time {
        s = s.filter(Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(Column::DictDataId)
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

pub async fn check_dict_data_is_exist(req: AddReq, db: &DatabaseConnection) -> Result<bool> {
    let s = SysDictData::find().filter(Column::DictType.eq(req.dict_type));
    let s1 = s.clone().filter(Column::DictValue.eq(req.dict_value));
    let s2 = s.clone().filter(Column::DictLabel.eq(req.dict_label));
    let count1 = s1.count(db).await?;
    let count2 = s2.count(db).await?;
    Ok(count1 > 0 || count2 > 0)
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
    if check_dict_data_is_exist(add_req.clone(), db).await? {
        return Err("字典标签已存在".into());
    }

    let uid = scru128::scru128();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = ActiveModel {
        dict_data_id: Set(uid.clone()),
        dict_label: Set(add_req.dict_label),
        dict_type: Set(add_req.dict_type),
        dict_sort: Set(add_req.dict_sort),
        is_default: Set(add_req.is_default),
        status: Set(add_req.status.unwrap_or(1)),
        remark: Set(Some(add_req.remark.unwrap_or("".to_string()))),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    let txn = db.begin().await?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysDictData::insert(user).exec(&txn).await?;
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
    let mut s = SysDictData::delete_many();

    s = s.filter(Column::DictDataId.is_in(delete_req.dict_ids));

    //开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => return Err("你要删除的字典类型不存在".into()),
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
    let uid = edit_req.dict_data_id;
    let s_s = SysDictData::find_by_id(uid.clone()).one(db).await?;
    let s_r: ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = ActiveModel {
        dict_label: Set(edit_req.dict_label),
        dict_type: Set(edit_req.dict_type),
        dict_sort: Set(edit_req.dict_sort),
        is_default: Set(edit_req.is_default),
        status: Set(edit_req.status),
        remark: Set(Some(edit_req.remark)),
        updated_at: Set(Some(now)),
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
    let mut s = SysDictData::find();
    s = s.filter(Column::DeletedAt.is_null());
    //
    if let Some(x) = search_req.dict_data_id {
        s = s.filter(Column::DictDataId.eq(x));
    } else {
        return Err("请输入字典类型id".into());
    }

    let res = match s.one(db).await? {
        Some(m) => m,
        None => return Err("该字典数据不存在".into()),
    };

    let result: Resp = serde_json::from_value(serde_json::json!(res))?; //这种数据转换效率不知道怎么样

    Ok(Json(serde_json::json!({ "result": result })))
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
#[handler]
pub async fn get_all(Data(db): Data<&DatabaseConnection>) -> Result<Json<serde_json::Value>> {
    let s = SysDictData::find()
        .filter(Column::DeletedAt.is_null())
        .filter(Column::Status.eq(1))
        .order_by(Column::DictDataId, Order::Asc)
        .all(db)
        .await?;
    let result: Vec<Resp> = serde_json::from_value(serde_json::json!(s))?; //这种数据转换效率不知道怎么样
    Ok(Json(serde_json::json!({ "result": result })))
}
