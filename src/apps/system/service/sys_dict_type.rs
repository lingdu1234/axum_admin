use crate::apps::common::models::{CudResData, ListData, PageParams, RespData};
use chrono::{Local, NaiveDateTime};
use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait,
    QueryFilter, QueryOrder, Set,
};
use serde_json::json;

use super::super::entities::{prelude::SysDictType, sys_dict_type};
use super::super::models::sys_dict_type::{AddReq, DeleteReq, EditReq, Resp, SearchReq};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    search_req: SearchReq,
) -> Result<ListData<sys_dict_type::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysDictType::find();

    if let Some(x) = search_req.dict_type {
        s = s.filter(sys_dict_type::Column::DictType.contains(&x));
    }

    if let Some(x) = search_req.dict_name {
        s = s.filter(sys_dict_type::Column::DictName.contains(&x));
    }
    if let Some(x) = search_req.status {
        s = s.filter(sys_dict_type::Column::Status.eq(x));
    }
    if let Some(x) = search_req.begin_time {
        s = s.filter(sys_dict_type::Column::CreatedAt.gte(x));
    }
    if let Some(x) = search_req.end_time {
        s = s.filter(sys_dict_type::Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await.map_err(BadRequest)?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_dict_type::Column::DictTypeId)
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

pub async fn check_dict_type_is_exist(dict_type: &str, db: &DatabaseConnection) -> Result<bool> {
    let mut s = SysDictType::find();
    s = s.filter(sys_dict_type::Column::DictType.eq(dict_type));
    let count = s.count(db).await.map_err(BadRequest)?;
    Ok(count > 0)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, add_req: AddReq) -> Result<RespData> {
    //  检查字典类型是否存在
    if check_dict_type_is_exist(&add_req.dict_type, db).await? {
        return Err(Error::from_string(
            "字典类型已存在",
            StatusCode::BAD_REQUEST,
        ));
    }

    let uid = scru128::scru128().to_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_dict_type::ActiveModel {
        dict_type_id: Set(uid.clone()),
        dict_name: Set(add_req.dict_name),
        dict_type: Set(add_req.dict_type),
        status: Set(add_req.status.unwrap_or_else(|| "1".to_string())),
        remark: Set(Some(add_req.remark.unwrap_or_else(|| "".to_string()))),
        created_at: Set(Some(now)),
        ..Default::default()
    };

    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysDictType::insert(user).exec(db).await.map_err(BadRequest);
    let res = json!({ "id": uid });
    Ok(RespData::with_data(res))
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<CudResData<String>> {
    let mut s = SysDictType::delete_many();

    s = s.filter(sys_dict_type::Column::DictTypeId.is_in(delete_req.dict_type_ids));

    //开始删除
    let d = s
        .exec(db)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::BAD_REQUEST))?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(Error::from_string(
            "你要删除的字典类型不存在",
            StatusCode::BAD_REQUEST,
        )),

        i => Ok(CudResData {
            id: None,
            msg: format!("成功删除{}条数据", i),
        }),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, edit_req: EditReq) -> Result<RespData> {
    let uid = edit_req.dict_type_id;
    let s_s = SysDictType::find_by_id(uid.clone())
        .one(db)
        .await
        .map_err(BadRequest)?;
    let s_r: sys_dict_type::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_dict_type::ActiveModel {
        dict_name: Set(edit_req.dict_name),
        dict_type: Set(edit_req.dict_type),
        status: Set(edit_req.status),
        remark: Set(Some(edit_req.remark)),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    let _aa = act.update(db).await.map_err(BadRequest)?; //这个两种方式一样 都要多查询一次

    Ok(RespData::with_msg(&format!("用户<{}>数据更新成功", uid)))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, search_req: SearchReq) -> Result<Resp> {
    let mut s = SysDictType::find();
    // s = s.filter(sys_dict_type::Column::DeletedAt.is_null());
    //
    if let Some(x) = search_req.dict_type_id {
        s = s.filter(sys_dict_type::Column::DictTypeId.eq(x));
    } else {
        return Err(Error::from_string(
            "请求参数错误,请输入Id",
            StatusCode::BAD_REQUEST,
        ));
    }

    let res = match s.into_model::<Resp>().one(db).await.map_err(BadRequest)? {
        Some(m) => m,
        None => return Err(Error::from_string("没有找到数据", StatusCode::BAD_REQUEST)),
    };
    // let result: Resp = serde_json::from_value(serde_json::json!(res)).map_err(BadRequest)?; //这种数据转换效率不知道怎么样
    Ok(res)
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Resp>> {
    let s = SysDictType::find()
        // .filter(sys_dict_type::Column::DeletedAt.is_null())
        .filter(sys_dict_type::Column::Status.eq(1))
        .order_by(sys_dict_type::Column::DictTypeId, Order::Asc)
        .into_model::<Resp>()
        .all(db)
        .await
        .map_err(BadRequest)?;
    // println!("{:?}", s);
    // let result: Vec<Resp> = serde_json::from_value(serde_json::json!(s)).map_err(BadRequest)?; //这种数据转换效率不知道怎么样
    Ok(s)
}
