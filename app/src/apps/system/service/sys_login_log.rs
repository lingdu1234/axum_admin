use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::{
        client::ClientInfo,
        res::{ListData, PageParams},
    },
    db_conn,
    system::{
        entities::{prelude::SysLoginLog, sys_login_log},
        models::sys_login_log::{SysLoginLogDeleteReq, SysLoginLogSearchReq},
        prelude::SysLoginLogModel,
    },
    DB,
};
use sea_orm::{sea_query::Table, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SysLoginLogSearchReq) -> Result<ListData<SysLoginLogModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysLoginLog::find();

    if let Some(x) = req.ip {
        if !x.is_empty() {
            s = s.filter(sys_login_log::Column::Ipaddr.contains(&x));
        }
    }
    if let Some(x) = req.user_name {
        if !x.is_empty() {
            s = s.filter(sys_login_log::Column::LoginName.contains(&x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_login_log::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_login_log::Column::LoginTime.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_login_log::Column::LoginTime.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let page = if let (Some(column), Some(order)) = (req.order_by_column, req.is_asc) {
        match (column.as_str(), order.as_str()) {
            ("login_name", "ascending") => s.order_by_asc(sys_login_log::Column::LoginName),
            ("login_name", "descending") => s.order_by_desc(sys_login_log::Column::LoginName),
            ("login_time", "ascending") => s.order_by_asc(sys_login_log::Column::LoginTime),
            ("login_time", "descending") => s.order_by_desc(sys_login_log::Column::LoginTime),
            (_, _) => s.order_by_desc(sys_login_log::Column::LoginTime),
        }
    } else {
        s.order_by_desc(sys_login_log::Column::LoginTime)
    };

    let paginator = page.paginate(db, page_per_size);
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
pub async fn delete(db: &DatabaseConnection, delete_req: SysLoginLogDeleteReq) -> Result<String> {
    let mut s = SysLoginLog::delete_many();

    s = s.filter(sys_login_log::Column::InfoId.is_in(delete_req.info_ids));

    // 开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在",)),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

pub async fn clean(db: &DatabaseConnection) -> Result<String> {
    let stmt = Table::truncate().table(sys_login_log::Entity).to_owned();
    let db_backend = db.get_database_backend();
    db.execute(db_backend.build(&stmt)).await?;
    Ok("登录日志已清空".to_string())
}

pub async fn add(req: ClientInfo, user: String, msg: String, status: String) {
    let db = DB.get_or_init(db_conn).await;
    let uid = scru128::new_string();
    let now = Local::now().naive_local();
    let active_model = sys_login_log::ActiveModel {
        info_id: Set(uid.clone()),
        login_name: Set(user.to_string()),
        net: Set(req.net.net_work),
        ipaddr: Set(req.net.ip),
        login_location: Set(req.net.location),
        browser: Set(req.ua.browser),
        os: Set(req.ua.os),
        device: Set(req.ua.device),
        status: Set(status.to_string()),
        msg: Set(msg.to_string()),
        login_time: Set(now),
        module: Set("系统后台".to_string()),
    };
    let txn = db.begin().await.expect("begin txn error");
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysLoginLog::insert(active_model).exec(db).await.expect("insert error");
    txn.commit().await.expect("commit txn error");
}
