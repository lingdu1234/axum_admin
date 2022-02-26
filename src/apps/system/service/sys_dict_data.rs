use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{prelude::SysDictData, sys_dict_data},
        models::sys_dict_data::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
    },
};
// use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::{
    ActiveModelTrait, ActiveValue::NotSet, ColumnTrait, ConnectionTrait, DatabaseConnection,
    EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    search_req: SearchReq,
) -> Result<ListData<sys_dict_data::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysDictData::find();

    if let Some(x) = search_req.dict_type {
        s = s.filter(sys_dict_data::Column::DictType.eq(x));
    }

    if let Some(x) = search_req.dict_label {
        s = s.filter(sys_dict_data::Column::DictLabel.eq(x));
    }
    if let Some(x) = search_req.status {
        s = s.filter(sys_dict_data::Column::Status.eq(x));
    }
    if let Some(x) = search_req.begin_time {
        s = s.filter(sys_dict_data::Column::CreatedAt.gte(x));
    }
    if let Some(x) = search_req.end_time {
        s = s.filter(sys_dict_data::Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_dict_data::Column::DictDataId)
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

pub async fn check_dict_data_is_exist<C>(req: AddReq, db: &C) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s = SysDictData::find().filter(sys_dict_data::Column::DictType.eq(req.dict_type));
    let s1 = s
        .clone()
        .filter(sys_dict_data::Column::DictValue.eq(req.dict_value));
    let s2 = s
        .clone()
        .filter(sys_dict_data::Column::DictLabel.eq(req.dict_label));
    let count1 = s1.count(db).await?;
    let count2 = s2.count(db).await?;
    Ok(count1 > 0 || count2 > 0)
}

/// add 添加
pub async fn add<C>(db: &C, add_req: AddReq, user_id: String) -> Result<String>
where
    C: TransactionTrait + ConnectionTrait,
{
    //  检查字典类型是否存在
    if check_dict_data_is_exist(add_req.clone(), db).await? {
        return Err(anyhow!("字典类型或者字典值或者字典标签已经存在"));
    }

    let uid = scru128::scru128().to_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_dict_data::ActiveModel {
        dict_data_id: Set(uid.clone()),
        dict_label: Set(add_req.dict_label),
        dict_type: Set(add_req.dict_type),
        dict_value: Set(add_req.dict_value),
        dict_sort: Set(add_req.dict_sort),
        is_default: Set(add_req.is_default),
        create_by: Set(user_id),
        css_class: match Some(add_req.css_class) {
            Some(x) => Set(x),
            None => NotSet,
        },
        list_class: match Some(add_req.list_class) {
            Some(x) => Set(x),
            None => NotSet,
        },
        status: Set(add_req.status.unwrap_or_else(|| "1".to_string())),
        remark: Set(Some(add_req.remark.unwrap_or_else(|| "".to_string()))),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    let txn = db.begin().await?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysDictData::insert(user).exec(&txn).await?;
    txn.commit().await?;
    let res = "数据添加成功".to_string();
    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = SysDictData::delete_many();

    s = s.filter(sys_dict_data::Column::DictDataId.is_in(delete_req.dict_data_ids));

    // 开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => Err(anyhow!("你要删除的字典类型不存在",)),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, edit_req: EditReq, user_id: String) -> Result<String> {
    let uid = edit_req.dict_data_id;
    let s_s = SysDictData::find_by_id(uid.clone()).one(db).await?;
    let s_r: sys_dict_data::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_dict_data::ActiveModel {
        dict_label: Set(edit_req.dict_label),
        dict_type: Set(edit_req.dict_type),
        dict_sort: Set(edit_req.dict_sort),
        dict_value: Set(edit_req.dict_value),
        update_by: Set(Some(user_id)),
        css_class: match Some(edit_req.css_class) {
            Some(x) => Set(x),
            None => NotSet,
        },
        list_class: match Some(edit_req.list_class) {
            Some(x) => Set(x),
            None => NotSet,
        },
        is_default: Set(edit_req.is_default),
        status: Set(edit_req.status),
        remark: Set(Some(edit_req.remark)),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    let _aa = act.update(db).await; // 这个两种方式一样 都要多查询一次

    Ok(format!("用户<{}>数据更新成功", uid))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0

pub async fn get_by_id(
    db: &DatabaseConnection,
    search_req: SearchReq,
) -> Result<sys_dict_data::Model> {
    let mut s = SysDictData::find();
    if let Some(x) = search_req.dict_data_id {
        s = s.filter(sys_dict_data::Column::DictDataId.eq(x));
    } else {
        return Err(anyhow!("请输入字典类型id",));
    }

    let res = match s.one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("字典类型不存在",)),
    };

    Ok(res)
}

pub async fn get_by_type(
    db: &DatabaseConnection,
    search_req: SearchReq,
) -> Result<Vec<sys_dict_data::Model>> {
    let mut s = SysDictData::find();
    if let Some(x) = search_req.dict_type {
        s = s.filter(sys_dict_data::Column::DictType.eq(x));
    } else {
        return Err(anyhow!("请输入字典类型",));
    }

    let res = s.all(db).await?;
    Ok(res)
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Resp>> {
    let s = SysDictData::find()
        // .filter(sys_dict_data::Column::DeletedAt.is_null())
        .filter(sys_dict_data::Column::Status.eq(1))
        .order_by(sys_dict_data::Column::DictDataId, Order::Asc)
        .into_model::<Resp>()
        .all(db)
        .await?;
    // let result: Vec<Resp> =
    // serde_json::from_value(serde_json::json!(s))?;
    // //这种数据转换效率不知道怎么样
    Ok(s)
}
