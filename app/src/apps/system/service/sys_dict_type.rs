use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{prelude::SysDictType, sys_dict_data, sys_dict_type},
        models::sys_dict_type::{SysDictTypeAddReq, SysDictTypeDeleteReq, SysDictTypeEditReq, SysDictTypeSearchReq},
        prelude::SysDictTypeModel,
    },
};
use sea_orm::{
    sea_query::{Expr, Query},
    ColumnTrait, Condition, ConnectionTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SysDictTypeSearchReq) -> Result<ListData<SysDictTypeModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysDictType::find();

    if let Some(x) = req.dict_type {
        s = s.filter(sys_dict_type::Column::DictType.contains(&x));
    }

    if let Some(x) = req.dict_name {
        s = s.filter(sys_dict_type::Column::DictName.contains(&x));
    }
    if let Some(x) = req.status {
        s = s.filter(sys_dict_type::Column::Status.eq(x));
    }
    if let Some(x) = req.begin_time {
        let x = x + " 00:00:00";
        let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
        s = s.filter(sys_dict_type::Column::CreatedAt.gte(t));
    }
    if let Some(x) = req.end_time {
        let x = x + " 23:59:59";
        let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
        s = s.filter(sys_dict_type::Column::CreatedAt.lte(t));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(sys_dict_type::Column::DictTypeId).paginate(db, page_per_size);
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
pub async fn add<C>(db: &C, req: SysDictTypeAddReq, user_id: String) -> Result<String>
where
    C: TransactionTrait + ConnectionTrait,
{
    //  检查字典类型是否存在
    if check_dict_type_is_exist(&req.dict_type, db).await? {
        return Err(anyhow!("字典类型已存在"));
    }
    let uid = scru128::new_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let dict_type = sys_dict_type::ActiveModel {
        dict_type_id: Set(uid.clone()),
        dict_name: Set(req.dict_name),
        dict_type: Set(req.dict_type),
        status: Set(req.status),
        remark: Set(req.remark),
        create_by: Set(user_id),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    SysDictType::insert(dict_type).exec(db).await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: SysDictTypeDeleteReq) -> Result<String> {
    // let count = SysDictType::find()
    //     .select_only()
    //     .column(sys_dict_type::Column::DictTypeId)
    //     .column(sys_dict_data::Column::DictType)
    //     .join_rev(
    //         JoinType::InnerJoin,
    //         sys_dict_data::Entity::belongs_to(sys_dict_type::Entity)
    //             .from(sys_dict_data::Column::DictType)
    //             .to(sys_dict_type::Column::DictType)
    //             .into(),
    //     )
    //     .filter(sys_dict_type::Column::DictTypeId.is_in(delete_req.dict_type_ids.
    // clone()))     .all(db)
    //     .await?;
    let count = sys_dict_data::Entity::find()
        .filter(
            Condition::any().add(
                sys_dict_data::Column::DictType.in_subquery(
                    Query::select()
                        .column(sys_dict_type::Column::DictType)
                        .from(sys_dict_type::Entity)
                        .and_where(Expr::col(sys_dict_type::Column::DictTypeId).is_in(delete_req.dict_type_ids.clone()))
                        .to_owned(),
                ),
            ),
        )
        .count(db)
        .await?;
    if count > 0 {
        return Err(anyhow!("存在关联数据，不能删除,请先删除关联字典数据"));
    }
    let txn = db.begin().await?;
    let mut s = SysDictType::delete_many();

    s = s.filter(sys_dict_type::Column::DictTypeId.is_in(delete_req.dict_type_ids));

    // 开始删除
    let d = s.exec(db).await?;
    txn.commit().await?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(anyhow!("你要删除的字典类型不存在")),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: SysDictTypeEditReq, user_id: String) -> Result<String> {
    sys_dict_type::Entity::update_many()
        .col_expr(sys_dict_type::Column::DictName, Expr::value(req.dict_name))
        .col_expr(sys_dict_type::Column::DictType, Expr::value(req.dict_type))
        .col_expr(sys_dict_type::Column::Status, Expr::value(req.status))
        .col_expr(sys_dict_type::Column::Remark, Expr::value(req.remark))
        .col_expr(sys_dict_type::Column::UpdateBy, Expr::value(user_id))
        .col_expr(sys_dict_type::Column::UpdatedAt, Expr::value(Local::now().naive_local()))
        .filter(sys_dict_type::Column::DictTypeId.eq(req.dict_type_id))
        .exec(db)
        .await?;
    Ok("数据更新成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id(db: &DatabaseConnection, req: SysDictTypeSearchReq) -> Result<SysDictTypeModel> {
    let mut s = SysDictType::find().filter(sys_dict_type::Column::DeletedAt.is_null());
    //
    if let Some(x) = req.dict_type_id {
        s = s.filter(sys_dict_type::Column::DictTypeId.eq(x));
    } else {
        return Err(anyhow!("请求参数错误,请输入Id"));
    }

    let res = match s.one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("没有找到数据")),
    };
    Ok(res)
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<SysDictTypeModel>> {
    let s = SysDictType::find()
        .filter(sys_dict_type::Column::DeletedAt.is_null())
        .filter(sys_dict_type::Column::Status.eq("1"))
        .order_by(sys_dict_type::Column::DictTypeId, Order::Asc)
        .all(db)
        .await?;
    Ok(s)
}
