use super::super::entities::{prelude::SysOperLog, sys_oper_log};
use super::super::models::sys_oper_log::{DeleteReq, SearchReq};
use crate::apps::common::models::{ListData, PageParams};
use crate::database::{db_conn, DB};
use crate::middleware::tracing_log::{ReqInfo, ResInfo};
use crate::utils::ApiUtils::ALL_APIS;
use chrono::Local;
use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    req: SearchReq,
) -> Result<ListData<sys_oper_log::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysOperLog::find();
    if let Some(x) = req.title {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::Title.eq(x));
        }
    }
    if let Some(x) = req.oper_name {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::OperName.contains(&x));
        }
    }

    if let Some(x) = req.operator_type {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::OperatorType.eq(x));
        }
    }

    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_oper_log::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        s = s.filter(sys_oper_log::Column::OperTime.gte(x));
    }
    if let Some(x) = req.end_time {
        s = s.filter(sys_oper_log::Column::OperTime.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await.map_err(BadRequest)?;
    // 分页获取数据
    let paginator = s
        .order_by_desc(sys_oper_log::Column::OperTime)
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

/// add 添加
pub async fn add(req: ReqInfo, res: ResInfo) -> Result<()> {
    let db = DB.get_or_init(db_conn).await;

    let operator_type = match req.clone().method.as_str() {
        "GET" => "1",    // 查询
        "POST" => "2",   // 新增
        "PUT" => "3",    // 修改
        "DELETE" => "4", // 删除
        _ => "0",        // 其他
    };
    let all_apis = ALL_APIS.lock().await;
    let req_path_vec = req.path.split("/").collect::<Vec<&str>>();
    let req_path = if req_path_vec.len() > 2 {
        req_path_vec[1..].join("/")
    } else {
        "".to_string()
    };
    println!(
        "=================++++++++++++++++++++++++++++++++++req_path: {}",
        req_path
    );
    let api_name = all_apis
        .get(&req_path)
        .unwrap_or(&("".to_string()))
        .to_string();

    let uid = scru128::scru128_string();
    let now = Local::now().naive_local();
    let add_data = sys_oper_log::ActiveModel {
        oper_id: Set(uid),
        time_id: Set(now.timestamp()),
        title: Set(api_name),
        business_type: Set("".to_string()),
        method: Set(req.path),
        request_method: Set(req.method),
        operator_type: Set(operator_type.to_string()),
        oper_name: Set(req.user),
        dept_name: Set("".to_string()),
        oper_url: Set(req.ori_path),
        oper_ip: Set(req.client_info.net.ip),
        oper_location: Set(req.client_info.net.location),
        oper_param: Set(req.data),
        url_param: Set(req.query),
        json_result: Set(res.data),
        status: Set(res.status),
        error_msg: Set(res.err_msg),
        oper_time: Set(now),
    };
    SysOperLog::insert(add_data)
        .exec(db)
        .await
        .map_err(BadRequest)?;

    Ok(())
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = SysOperLog::delete_many();

    s = s.filter(sys_oper_log::Column::OperId.is_in(delete_req.oper_log_ids));

    //开始删除
    let d = s
        .exec(db)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::BAD_REQUEST))?;

    match d.rows_affected {
        // 0 => return Err("你要删除的字典类型不存在".into()),
        0 => Err(Error::from_string(
            "你要删除的日志不存在".to_string(),
            StatusCode::BAD_REQUEST,
        )),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}

/// delete 完全删除
pub async fn clean(db: &DatabaseConnection) -> Result<String> {
    SysOperLog::delete_many()
        .exec(db)
        .await
        .map_err(|e| Error::from_string(e.to_string(), StatusCode::BAD_REQUEST))?;

    Ok("日志清空成功".to_string())
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, oper_id: String) -> Result<sys_oper_log::Model> {
    let s = SysOperLog::find()
        .filter(sys_oper_log::Column::OperId.eq(oper_id))
        .one(db)
        .await
        .map_err(BadRequest)?;

    let res = match s {
        Some(m) => m,
        None => return Err(Error::from_string("没有找到数据", StatusCode::BAD_REQUEST)),
    };
    Ok(res)
}
