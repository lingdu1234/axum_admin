use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::{
        client::ClientInfo,
        res::{ListData, PageParams},
    },
    db_conn,
    system::{
        entities::{prelude::SysUserOnline, sys_user_online},
        models::sys_user_online::{SysUserOnlineDeleteReq, SysUserOnlineSearchReq},
        prelude::SysUserOnlineModel,
    },
    DB,
};
use sea_orm::{sea_query::Expr, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};

/// get_list 获取列表
/// page_params 分页参数
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SysUserOnlineSearchReq) -> Result<ListData<SysUserOnlineModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysUserOnline::find();

    if let Some(x) = req.ipaddr {
        if !x.is_empty() {
            s = s.filter(sys_user_online::Column::Ipaddr.contains(&x));
        }
    }
    if let Some(x) = req.user_name {
        if !x.is_empty() {
            s = s.filter(sys_user_online::Column::UserName.contains(&x));
        }
    }

    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_user_online::Column::LoginTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_user_online::Column::LoginTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_desc(sys_user_online::Column::LoginTime).paginate(db, page_per_size);

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

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: SysUserOnlineDeleteReq) -> Result<String> {
    let mut s = SysUserOnline::delete_many();

    s = s.filter(sys_user_online::Column::Id.is_in(delete_req.ids));

    // 开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在")),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn check_online(db: Option<&DatabaseConnection>, id: String) -> (bool, Option<SysUserOnlineModel>) {
    let db = match db {
        Some(x) => x,
        None => DB.get_or_init(db_conn).await,
    };

    let model = SysUserOnline::find().filter(sys_user_online::Column::TokenId.eq(id)).one(db).await.expect("查询失败");

    (model.is_some(), model)
}

pub async fn log_out(db: &DatabaseConnection, token_id: String) -> Result<String> {
    let s = SysUserOnline::delete_many().filter(sys_user_online::Column::TokenId.eq(token_id));
    s.exec(db).await?;
    Ok("成功退出登录".to_string())
}

pub async fn add(req: ClientInfo, u_id: String, token_id: String, token_exp: i64) {
    let db = DB.get_or_init(db_conn).await;
    let uid = scru128::new_string();
    let now = Local::now().naive_local();
    let user = super::sys_user::get_by_id(db, &u_id).await.expect("获取用户信息失败");
    let dept = super::sys_dept::get_by_id(db, &user.clone().user.dept_id).await.expect("获取部门信息失败");
    let active_model = sys_user_online::ActiveModel {
        id: Set(uid.clone()),
        u_id: Set(u_id),
        token_id: Set(token_id),
        token_exp: Set(token_exp),
        user_name: Set(user.user.user_name.clone()),
        dept_name: Set(dept.dept_name.clone()),
        net: Set(req.net.net_work),
        ipaddr: Set(req.net.ip),
        login_location: Set(req.net.location),
        browser: Set(req.ua.browser),
        os: Set(req.ua.os),
        device: Set(req.ua.device),
        login_time: Set(now),
    };
    let txn = db.begin().await.expect("begin txn error");
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysUserOnline::insert(active_model).exec(&txn).await.expect("insert error");
    txn.commit().await.expect("commit txn error");
}

pub async fn update_online(token_id: String, token_exp: i64) -> Result<String> {
    let db = DB.get_or_init(db_conn).await;
    let txn = db.begin().await?;
    SysUserOnline::update_many()
        .col_expr(sys_user_online::Column::TokenExp, Expr::value(token_exp))
        .filter(sys_user_online::Column::TokenId.eq(token_id))
        .exec(&txn)
        .await?;
    txn.commit().await?;
    Ok("token更新成功".to_string())
}
