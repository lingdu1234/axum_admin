use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{prelude::*, sys_post, sys_user_post},
        models::sys_post::{AddReq, DeleteReq, EditReq, Resp, SearchReq},
    },
};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SearchReq) -> Result<ListData<sys_post::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysPost::find();

    if let Some(x) = req.post_code {
        s = s.filter(sys_post::Column::PostCode.contains(&x));
    }

    if let Some(x) = req.post_name {
        s = s.filter(sys_post::Column::PostName.contains(&x));
    }
    if let Some(x) = req.status {
        s = s.filter(sys_post::Column::Status.eq(x));
    }
    if let Some(x) = req.begin_time {
        s = s.filter(sys_post::Column::CreatedAt.gte(x));
    }
    if let Some(x) = req.end_time {
        s = s.filter(sys_post::Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(sys_post::Column::PostId).paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        total,
        total_pages,
        list,
        page_num,
    };
    Ok(res)
}

pub async fn check_data_is_exist(post_code: String, post_name: String, db: &DatabaseConnection) -> Result<bool> {
    let s1 = SysPost::find().filter(sys_post::Column::PostCode.eq(post_code));
    let s2 = SysPost::find().filter(sys_post::Column::PostName.eq(post_name));
    let count1 = s1.count(db).await?;
    let count2 = s2.count(db).await?;
    Ok(count1 > 0 || count2 > 0)
}

pub async fn eidt_check_data_is_exist(post_id: String, post_code: String, post_name: String, db: &DatabaseConnection) -> Result<bool> {
    let count1 = SysPost::find()
        .filter(sys_post::Column::PostCode.eq(post_code))
        .filter(sys_post::Column::PostId.ne(post_id.clone()))
        .count(db)
        .await?;
    let count2 = SysPost::find()
        .filter(sys_post::Column::PostName.eq(post_name))
        .filter(sys_post::Column::PostId.ne(post_id))
        .count(db)
        .await?;
    Ok(count1 > 0 || count2 > 0)
}

/// add 添加

pub async fn add(db: &DatabaseConnection, req: AddReq, user_id: String) -> Result<String> {
    //  检查字典类型是否存在
    if check_data_is_exist(req.clone().post_code, req.clone().post_name, db).await? {
        return Err(anyhow!("数据已存在"));
    }

    let uid = scru128::scru128().to_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_post::ActiveModel {
        post_id: Set(uid.clone()),
        post_code: Set(req.post_code),
        post_sort: Set(req.post_sort),
        post_name: Set(req.post_name),
        status: Set(req.status.unwrap_or_else(|| "1".to_string())),
        remark: Set(Some(req.remark.unwrap_or_else(|| "".to_string()))),
        created_by: Set(user_id),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    let txn = db.begin().await?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysPost::insert(user).exec(&txn).await?;
    txn.commit().await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = SysPost::delete_many();

    s = s.filter(sys_post::Column::PostId.is_in(delete_req.post_ids));

    // 开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, edit_req: EditReq, user_id: String) -> Result<String> {
    //  检查字典类型是否存在
    if eidt_check_data_is_exist(edit_req.clone().post_id, edit_req.clone().post_code, edit_req.clone().post_name, db).await? {
        return Err(anyhow!("数据已存在"));
    }
    let uid = edit_req.post_id;
    let s_s = SysPost::find_by_id(uid.clone()).one(db).await?;
    let s_r: sys_post::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_post::ActiveModel {
        post_code: Set(edit_req.post_code),
        post_name: Set(edit_req.post_name),
        post_sort: Set(edit_req.post_sort),
        status: Set(edit_req.status),
        remark: Set(Some(edit_req.remark)),
        updated_by: Set(Some(user_id)),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    let _aa = act.update(db).await?; // 这个两种方式一样 都要多查询一次
    Ok(format!("用户<{}>数据更新成功", uid))
}

/// get_user_by_id 获取用户Id获取用户
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, search_req: SearchReq) -> Result<Resp> {
    let mut s = SysPost::find();
    s = s.filter(sys_post::Column::DeletedAt.is_null());
    //
    if let Some(x) = search_req.post_id {
        s = s.filter(sys_post::Column::PostId.eq(x));
    } else {
        return Err(anyhow!("请求参数错误"));
    }

    let res = match s.into_model::<Resp>().one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("数据不存在")),
    };

    // let result: Resp =
    // serde_json::from_value(serde_json::json!(res))?;
    // //这种数据转换效率不知道怎么样

    Ok(res)
}

pub async fn get_post_ids_by_user_id(db: &DatabaseConnection, user_id: &str) -> Result<Vec<String>> {
    let s = SysUserPost::find().filter(sys_user_post::Column::UserId.eq(user_id)).all(db).await?;

    let mut res = Vec::new();

    for x in s {
        res.push(x.post_id);
    }

    Ok(res)
}

/// get_all 获取全部
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Resp>> {
    let s = SysPost::find()
        .filter(sys_post::Column::DeletedAt.is_null())
        .filter(sys_post::Column::Status.eq(1))
        .order_by(sys_post::Column::PostId, Order::Asc)
        .into_model::<Resp>()
        .all(db)
        .await?;
    Ok(s)
}

pub async fn delete_post_by_user_id<C>(db: &C, user_ids: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    SysUserPost::delete_many().filter(sys_user_post::Column::UserId.is_in(user_ids)).exec(db).await?;
    Ok(())
}

pub async fn add_post_by_user_id<C>(db: &C, user_id: &str, post: Vec<String>) -> Result<()>
where
    C: TransactionTrait + ConnectionTrait,
{
    let mut inser_data: Vec<sys_user_post::ActiveModel> = Vec::new();
    for x in post {
        let now: NaiveDateTime = Local::now().naive_local();
        let act = sys_user_post::ActiveModel {
            user_id: Set(user_id.to_string()),
            post_id: Set(x),
            created_at: Set(Some(now)),
        };
        inser_data.push(act);
    }
    SysUserPost::insert_many(inser_data).exec(db).await?;
    Ok(())
}
