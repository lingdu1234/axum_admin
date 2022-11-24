use anyhow::{anyhow, Result};
use chrono::{Local, NaiveDateTime};
use db::{
    common::res::{ListData, PageParams},
    system::{
        entities::{
            prelude::{SysDept, SysRoleDept},
            sys_dept, sys_role_dept,
        },
        models::sys_dept::{DeptResp, RespTree, SysDeptAddReq, SysDeptDeleteReq, SysDeptEditReq, SysDeptSearchReq},
        prelude::SysDeptModel,
    },
};
use sea_orm::{ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, req: SysDeptSearchReq) -> Result<ListData<SysDeptModel>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = SysDept::find();

    if let Some(x) = req.dept_id {
        if !x.is_empty() {
            s = s.filter(sys_dept::Column::DeptId.eq(x));
        }
    }

    if let Some(x) = req.dept_name {
        if !x.is_empty() {
            s = s.filter(sys_dept::Column::DeptName.contains(&x));
        }
    }
    if let Some(x) = req.status {
        if !x.is_empty() {
            s = s.filter(sys_dept::Column::Status.eq(x));
        }
    }
    if let Some(x) = req.begin_time {
        if !x.is_empty() {
            let x = x + " 00:00:00";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_dept::Column::CreatedAt.gte(t));
        }
    }
    if let Some(x) = req.end_time {
        if !x.is_empty() {
            let x = x + " 23:59:59";
            let t = NaiveDateTime::parse_from_str(&x, "%Y-%m-%d %H:%M:%S")?;
            s = s.filter(sys_dept::Column::CreatedAt.lte(t));
        }
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(sys_dept::Column::OrderNum).paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        total,
        page_num,
        list,
        total_pages,
    };
    Ok(res)
}

pub async fn check_data_is_exist(dept_name: String, db: &DatabaseConnection) -> Result<bool> {
    let s1 = SysDept::find().filter(sys_dept::Column::DeptName.eq(dept_name));
    let count1 = s1.count(db).await?;
    Ok(count1 > 0)
}

/// add 添加

pub async fn add(db: &DatabaseConnection, req: SysDeptAddReq, user_id: String) -> Result<String> {
    //  检查字典类型是否存在
    if check_data_is_exist(req.clone().dept_name, db).await? {
        return Err(anyhow!("数据已存在",));
    }

    let uid = scru128::new_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let user = sys_dept::ActiveModel {
        dept_id: Set(uid.clone()),
        parent_id: Set(req.parent_id),
        dept_name: Set(req.dept_name),
        order_num: Set(req.order_num),
        leader: Set(req.leader),
        status: Set(req.status),
        phone: Set(req.phone),
        email: Set(req.email),
        created_by: Set(user_id),
        created_at: Set(now),
        ..Default::default()
    };
    let txn = db.begin().await?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysDept::insert(user).exec(&txn).await?;
    txn.commit().await?;
    let res = "添加成功".to_string();

    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, req: SysDeptDeleteReq) -> Result<String> {
    let d = SysDept::delete_many().filter(sys_dept::Column::DeptId.eq(req.dept_id)).exec(db).await?;

    match d.rows_affected {
        0 => Err(anyhow!("删除失败,数据不存在",)),
        i => Ok(format!("成功删除{}条数据", i)),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: SysDeptEditReq, user_id: String) -> Result<String> {
    //  检查字典类型是否存在
    let s1 = SysDept::find()
        .filter(sys_dept::Column::DeptName.eq(req.clone().dept_name))
        .filter(sys_dept::Column::DeptId.ne(req.clone().dept_id))
        .count(db)
        .await?;
    if s1 > 0 {
        return Err(anyhow!("数据已存在",));
    }
    let uid = req.dept_id;
    let s_s = SysDept::find_by_id(uid.clone()).one(db).await?;
    let s_r: sys_dept::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_dept::ActiveModel {
        parent_id: Set(req.parent_id),
        dept_name: Set(req.dept_name),
        order_num: Set(req.order_num),
        status: Set(req.status),
        leader: Set(req.leader),
        phone: Set(req.phone),
        email: Set(req.email),
        updated_by: Set(Some(user_id)),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    act.update(db).await?; // 这个两种方式一样 都要多查询一次
    Ok(format!("用户<{}>数据更新成功", uid))
}

/// get_user_by_id 获取用户Id获取用户
pub async fn get_by_id<C>(db: &C, id: &str) -> Result<DeptResp>
where
    C: ConnectionTrait + TransactionTrait,
{
    let s = SysDept::find().filter(sys_dept::Column::DeletedAt.is_null()).filter(sys_dept::Column::DeptId.eq(id));

    let res = match s.into_model::<DeptResp>().one(db).await? {
        Some(m) => m,
        None => return Err(anyhow!("数据不存在",)),
    };

    Ok(res)
}

/// get_all 获取全部
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<DeptResp>> {
    let s = SysDept::find()
        .filter(sys_dept::Column::DeletedAt.is_null())
        .filter(sys_dept::Column::Status.eq("1"))
        .order_by(sys_dept::Column::OrderNum, Order::Asc)
        .into_model::<DeptResp>()
        .all(db)
        .await?;
    Ok(s)
}

pub async fn get_dept_tree(db: &DatabaseConnection) -> Result<Vec<RespTree>> {
    // 获取全部数据
    let dept_list = get_all(db).await.unwrap();

    // 创建树
    let mut tree: Vec<RespTree> = Vec::new();
    for item in dept_list {
        let tree_item = RespTree { data: item, ..Default::default() };
        tree.push(tree_item);
    }
    let res = create_menu_tree(tree, "0".to_string());
    Ok(res)
}

pub fn create_menu_tree(depts: Vec<RespTree>, pid: String) -> Vec<RespTree> {
    let mut tree: Vec<RespTree> = Vec::new();
    for mut t in depts.clone() {
        if t.data.parent_id == pid {
            t.children = Some(create_menu_tree(depts.clone(), t.data.dept_id.clone()));
            tree.push(t.clone());
        }
    }
    tree
}

//  根据角色id获取授权部门
pub async fn get_dept_by_role_id(db: &DatabaseConnection, role_id: String) -> Result<Vec<String>> {
    let mut dept_ids: Vec<String> = Vec::new();
    let mut s = SysRoleDept::find();
    s = s.filter(sys_role_dept::Column::RoleId.eq(role_id));
    let res = s.all(db).await?;
    for item in res {
        dept_ids.push(item.dept_id);
    }
    Ok(dept_ids)
}
