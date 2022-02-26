use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{prelude::SysDictType, sys_dict_type},
        models::sys_dict_type::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
    },
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

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
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_dict_type::Column::DictTypeId)
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

pub async fn check_dict_type_is_exist<C>(dict_type: &str, db: &C) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let mut s = SysDictType::find();
    s = s.filter(sys_dict_type::Column::DictType.eq(dict_type));
    let count = s.count(db).await?;
    Ok(count > 0)
}

/// add 添加
pub async fn add<C>(db: &C, req: AddReq, user_id: String) -> Result<String>
where
    C: TransactionTrait + ConnectionTrait,
{
    //  检查字典类型是否存在
    if check_dict_type_is_exist(&req.dict_type, db).await? {
        return Err(anyhow!("字典类型已存在"));
    }
    let uid = scru128::scru128_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let dict_type = sys_dict_type::ActiveModel {
        dict_type_id: Set(uid.clone()),
        dict_name: Set(req.dict_name),
        dict_type: Set(req.dict_type),
        status: Set(req.status.unwrap_or_else(|| "1".to_string())),
        remark: Set(Some(req.remark.unwrap_or_else(|| "".to_string()))),
        create_by: Set(user_id),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    SysDictType::insert(dict_type).exec(db).await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = SysDictType::delete_many();

    s = s.filter(sys_dict_type::Column::DictTypeId.is_in(delete_req.dict_type_ids));

    // 开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(anyhow!("你要删除的字典类型不存在")),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, edit_req: EditReq, user_id: String) -> Result<String> {
    let uid = edit_req.dict_type_id;
    let s_s = SysDictType::find_by_id(uid.clone()).one(db).await?;
    let s_r: sys_dict_type::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_dict_type::ActiveModel {
        dict_name: Set(edit_req.dict_name),
        dict_type: Set(edit_req.dict_type),
        status: Set(edit_req.status),
        remark: Set(Some(edit_req.remark)),
        update_by: Set(Some(user_id)),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    let _aa = act.update(db).await?; // 这个两种方式一样 都要多查询一次

    Ok(format!("用户<{}>数据更新成功", uid))
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, req: SearchReq) -> Result<Resp> {
    let mut s = SysDictType::find();
    // s = s.filter(sys_dict_type::Column::DeletedAt.is_null());
    //
    if let Some(x) = req.dict_type_id {
        s = s.filter(sys_dict_type::Column::DictTypeId.eq(x));
    } else {
        return Err(anyhow!("请求参数错误,请输入Id"));
    }

    let res = match s.into_model::<Resp>().one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("没有找到数据")),
    };
    // let result: Resp =
    // serde_json::from_value(serde_json::json!(res))?;
    // //这种数据转换效率不知道怎么样
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
        .await?;
    // println!("{:?}", s);
    // let result: Vec<Resp> =
    // serde_json::from_value(serde_json::json!(s))?;
    // //这种数据转换效率不知道怎么样
    Ok(s)
}
