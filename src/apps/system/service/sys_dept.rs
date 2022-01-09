use crate::apps::common::models::{PageParams, RespData};
use chrono::{Local, NaiveDateTime};
use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use serde_json::json;

use crate::apps::system::models::sys_dept::RespTree;

use super::super::entities::{prelude::*, sys_dept};
use super::super::models::sys_dept::{AddReq, DeleteReq, EditReq, Resp, SearchReq};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    search_req: SearchReq,
) -> Result<RespData> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysDept::find();

    if let Some(x) = search_req.dept_id {
        s = s.filter(sys_dept::Column::DeptId.eq(x));
    }

    if let Some(x) = search_req.dept_name {
        s = s.filter(sys_dept::Column::DeptName.eq(x));
    }
    if let Some(x) = search_req.status {
        s = s.filter(sys_dept::Column::Status.eq(x));
    }
    if let Some(x) = search_req.begin_time {
        s = s.filter(sys_dept::Column::CreatedAt.gte(x));
    }
    if let Some(x) = search_req.end_time {
        s = s.filter(sys_dept::Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await.map_err(BadRequest)?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_dept::Column::OrderNum)
        .paginate(db, page_per_size);
    let num_pages = paginator.num_pages().await.map_err(BadRequest)?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(BadRequest)?;

    let res = json!({
            "list": list,
            "total": total,
            "total_pages": num_pages,
            "page_num": page_num,
    });
    Ok(RespData::with_data(res))
}

pub async fn check_data_is_exist(dept_name: String, db: &DatabaseConnection) -> Result<bool> {
    let s1 = SysDept::find().filter(sys_dept::Column::DeptName.eq(dept_name));
    let count1 = s1.count(db).await.map_err(BadRequest)?;
    Ok(count1 > 0)
}

/// add 添加

pub async fn add(db: &DatabaseConnection, add_req: AddReq) -> Result<RespData> {
    //  检查字典类型是否存在
    if check_data_is_exist(add_req.clone().dept_name, db).await? {
        return Err(Error::from_string("数据已存在", StatusCode::BAD_REQUEST));
    }

    let uid = scru128::scru128().to_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_dept::ActiveModel {
        dept_id: Set(uid.clone()),
        dept_name: Set(add_req.dept_name),
        order_num: Set(add_req.order_num),
        status: Set(add_req.status),
        phone: Set(add_req.phone),
        email: Set(add_req.email),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    let txn = db.begin().await.map_err(BadRequest)?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysDept::insert(user).exec(&txn).await.map_err(BadRequest)?;
    txn.commit().await.map_err(BadRequest)?;
    let res = json!({ "id": uid });
    Ok(RespData::with_data(res))
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<RespData> {
    let mut s = SysDept::delete_many();

    s = s.filter(sys_dept::Column::DeptId.is_in(delete_req.dept_ids));

    //开始删除
    let d = s.exec(db).await.map_err(BadRequest)?;

    match d.rows_affected {
        0 => Err(Error::from_string(
            "删除失败,数据不存在",
            StatusCode::BAD_REQUEST,
        )),
        i => Ok(RespData::with_msg(&format!("成功删除{}条数据", i))),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, edit_req: EditReq) -> Result<RespData> {
    //  检查字典类型是否存在
    if check_data_is_exist(edit_req.clone().dept_name, db).await? {
        return Err(Error::from_string("数据已存在", StatusCode::BAD_REQUEST));
    }
    let uid = edit_req.dept_id;
    let s_s = SysDept::find_by_id(uid.clone())
        .one(db)
        .await
        .map_err(BadRequest)?;
    let s_r: sys_dept::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_dept::ActiveModel {
        dept_name: Set(edit_req.dept_name),
        order_num: Set(edit_req.order_num),
        status: Set(edit_req.status),
        phone: Set(edit_req.phone),
        email: Set(edit_req.email),
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
    let mut s = SysDept::find();
    s = s.filter(sys_dept::Column::DeletedAt.is_null());
    //
    if let Some(x) = search_req.dept_id {
        s = s.filter(sys_dept::Column::DeptId.eq(x));
    } else {
        return Err(Error::from_string("请求参数错误", StatusCode::BAD_REQUEST));
    }

    let res = match s.into_model::<Resp>().one(db).await.map_err(BadRequest)? {
        Some(m) => m,
        None => return Err(Error::from_string("数据不存在", StatusCode::BAD_REQUEST)),
    };

    // let result: Resp = serde_json::from_value(serde_json::json!(res)).map_err(BadRequest)?; //这种数据转换效率不知道怎么样

    Ok(res)
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Resp>> {
    let s = SysDept::find()
        .filter(sys_dept::Column::DeletedAt.is_null())
        .filter(sys_dept::Column::Status.eq(1))
        .order_by(sys_dept::Column::OrderNum, Order::Asc)
        .into_model::<Resp>()
        .all(db)
        .await
        .map_err(BadRequest)?;
    Ok(s)
}

pub async fn get_dept_tree(db: &DatabaseConnection) -> Result<Vec<RespTree>> {
    // 获取全部数据
    let dept_list = get_all(db).await.unwrap();
    // 创建树
    let mut tree: Vec<RespTree> = Vec::new();
    for item in dept_list {
        let tree_item = RespTree {
            data: item,
            ..Default::default()
        };
        tree.push(tree_item);
    }
    let res = creat_menu_tree(tree, "0".to_string());
    Ok(res)
}

pub fn creat_menu_tree(depts: Vec<RespTree>, pid: String) -> Vec<RespTree> {
    let mut tree: Vec<RespTree> = Vec::new();
    for mut t in depts.clone() {
        if t.data.dept_id == pid {
            t.children = Some(creat_menu_tree(depts.clone(), t.data.dept_id.clone()));
            tree.push(t.clone());
        }
    }
    tree
}
