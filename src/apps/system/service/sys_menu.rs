use crate::apps::common::models::{CudResData, ListData, PageParams};
use crate::utils::{self, get_enforcer};
use chrono::{Local, NaiveDateTime};
use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set,TransactionTrait
};
use sea_orm_casbin_adapter::casbin::MgmtApi;

use super::super::entities::{prelude::*, sys_menu};
use super::super::models::sys_menu::{
    AddReq, DeleteReq, EditReq, MenuResp, Meta, SearchReq, SysMenuTree, UserMenu,
};

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(
    db: &DatabaseConnection,
    page_params: PageParams,
    search_req: SearchReq,
) -> Result<ListData<sys_menu::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(usize::MAX);
    //  生成查询条件
    let mut s = SysMenu::find();

    if let Some(x) = search_req.menu_name {
        s = s.filter(sys_menu::Column::MenuName.contains(&x));
    }

    if let Some(x) = search_req.status {
        s = s.filter(sys_menu::Column::Status.eq(x));
    }
    if let Some(x) = search_req.menu_type {
        s = s.filter(sys_menu::Column::MenuType.eq(x));
    }
    if let Some(x) = search_req.begin_time {
        s = s.filter(sys_menu::Column::CreatedAt.gte(x));
    }
    if let Some(x) = search_req.end_time {
        s = s.filter(sys_menu::Column::CreatedAt.lte(x));
    }
    // 获取全部数据条数
    let total = s.clone().count(db).await.map_err(BadRequest)?;
    // 分页获取数据
    let paginator = s
        .order_by_asc(sys_menu::Column::OrderSort)
        .paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await.map_err(BadRequest)?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(BadRequest)?;

    let res = ListData {
        list,
        total,
        total_pages,
        page_num,
    };
    Ok(res)
}

pub async fn check_router_is_exist_update(
    db: &DatabaseConnection,
    id: String,
    route_path: String,
    route_name: String,
) -> Result<bool> {
    let s1 = SysMenu::find()
        .filter(sys_menu::Column::Path.eq(route_path))
        .filter(sys_menu::Column::MenuType.ne("F"))
        .filter(sys_menu::Column::Id.ne(id.clone()));
    let count1 = s1.count(db).await.map_err(BadRequest)?;
    let s2 = SysMenu::find()
        .filter(sys_menu::Column::MenuName.eq(route_name))
        .filter(sys_menu::Column::MenuType.ne("F"))
        .filter(sys_menu::Column::Id.ne(id));
    let count2 = s2.count(db).await.map_err(BadRequest)?;
    Ok(count1 > 0 || count2 > 0)
}

pub async fn check_router_is_exist_add<'a, C>(
    db: &'a C,
    route_path: String,
    route_name: String,
) -> Result<bool>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s1 = SysMenu::find()
        .filter(sys_menu::Column::Path.eq(route_path))
        .filter(sys_menu::Column::MenuType.ne("F"));
    let count1 = s1.count(db).await.map_err(BadRequest)?;
    let s2 = SysMenu::find()
        .filter(sys_menu::Column::MenuName.eq(route_name))
        .filter(sys_menu::Column::MenuType.ne("F"));
    let count2 = s2.count(db).await.map_err(BadRequest)?;
    Ok(count1 > 0 || count2 > 0)
}

/// add 添加
pub async fn add<'a, C>(db: &'a C, req: AddReq) -> Result<CudResData<String>>
where
    C: TransactionTrait + ConnectionTrait,
{
    //  检查数据是否存在
    if check_router_is_exist_add(
        db,
        req.clone().path.unwrap_or_else(|| "".to_string()),
        req.clone().menu_name,
    )
    .await?
    {
        return Err(Error::from_string(
            "路径或者名称重复",
            StatusCode::BAD_REQUEST,
        ));
    }
    let reqq = req.clone();
    let uid = scru128::scru128().to_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let active_model = sys_menu::ActiveModel {
        id: Set(uid.clone()),
        pid: Set(req.pid),
        menu_name: Set(req.menu_name),
        icon: Set(req.icon.unwrap_or_else(|| "".to_string())),
        remark: Set(req.remark),
        menu_type: Set(req.menu_type),
        query: Set(req.query),
        api: Set(req.api.unwrap_or_else(|| "".to_string())),
        method: Set(req.method.unwrap_or_else(|| "".to_string())),
        order_sort: Set(req.order_sort),
        status: Set(req.status),
        visible: Set(req.visible),
        path: Set(req.path.unwrap_or_else(|| "".to_string())),
        component: Set(req.component.unwrap_or_else(|| "".to_string())),
        is_data_scope: Set(req.is_data_scope),
        is_frame: Set(req.is_frame),
        is_cache: Set(req.is_cache),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    let txn = db.begin().await.map_err(BadRequest)?;
    //  let re =   user.insert(db).await?; 这个多查询一次结果
    let _ = SysMenu::insert(active_model)
        .exec(&txn)
        .await
        .map_err(BadRequest)?;
    txn.commit().await.map_err(BadRequest)?;
    let res = CudResData {
        id: Some(uid.clone()),
        msg: format!("{} 添加成功", uid),
    };
    // 添加api到全局
    utils::ApiUtils::add_api(reqq.api.unwrap().as_str()).await;
    Ok(res)
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<CudResData<String>> {
    let mut s = SysMenu::delete_many();

    s = s.filter(sys_menu::Column::Id.is_in(delete_req.ids));

    //开始删除
    let d = s.exec(db).await.map_err(BadRequest)?;

    match d.rows_affected {
        0 => Err(Error::from_string(
            "删除失败,数据不存在",
            StatusCode::BAD_REQUEST,
        )),
        i => Ok(CudResData {
            id: None,
            msg: format!("删除成功,共删除 {} 条数据", i),
        }),
    }
}

// edit 修改
pub async fn edit(db: &DatabaseConnection, req: EditReq) -> Result<CudResData<String>> {
    //  检查数据是否存在
    if check_router_is_exist_update(db, req.clone().id, req.clone().path, req.clone().menu_name)
        .await?
    {
        return Err(Error::from_string(
            "路径或者名称重复",
            StatusCode::BAD_REQUEST,
        ));
    }
    let reqq = req.clone();
    let uid = req.id;
    let s_s = SysMenu::find_by_id(uid.clone())
        .one(db)
        .await
        .map_err(BadRequest)?;
    let s_r: sys_menu::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_menu::ActiveModel {
        id: Set(uid.clone()),
        pid: Set(req.pid),
        menu_name: Set(req.menu_name),
        icon: Set(req.icon.unwrap_or_else(|| "".to_string())),
        remark: Set(req.remark),
        api: Set(req.api.unwrap_or_else(|| "".to_string())),
        method: Set(req.method.unwrap_or_else(|| "".to_string())),
        query: Set(req.query),
        menu_type: Set(req.menu_type),
        order_sort: Set(req.order_sort),
        status: Set(req.status),
        visible: Set(req.visible),
        path: Set(req.path),
        component: Set(req.component),
        is_data_scope: Set(req.is_data_scope),
        is_frame: Set(req.is_frame),
        is_cache: Set(req.is_cache),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    let _aa = act.update(db).await.map_err(BadRequest)?; //这个两种方式一样 都要多查询一次
    utils::ApiUtils::add_api(reqq.api.unwrap().as_str()).await;
    let res = CudResData {
        id: Some(uid.clone()),
        msg: format!("{} 修改成功", uid),
    };
    Ok(res)
}

/// get_user_by_id 获取用户Id获取用户   
/// db 数据库连接 使用db.0
pub async fn get_by_id(db: &DatabaseConnection, search_req: SearchReq) -> Result<MenuResp> {
    let mut s = SysMenu::find();
    s = s.filter(sys_menu::Column::DeletedAt.is_null());
    //
    if let Some(x) = search_req.id {
        s = s.filter(sys_menu::Column::Id.eq(x));
    } else {
        return Err(Error::from_string("请求参数错误", StatusCode::BAD_REQUEST));
    }

    let res = match s
        .into_model::<MenuResp>()
        .one(db)
        .await
        .map_err(BadRequest)?
    {
        Some(m) => m,
        None => return Err(Error::from_string("数据不存在", StatusCode::BAD_REQUEST)),
    };

    Ok(res)
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_all<'a, C>(db: &'a C) -> Result<Vec<MenuResp>>
where
    C: TransactionTrait + ConnectionTrait,
{
    let s = SysMenu::find()
        .filter(sys_menu::Column::DeletedAt.is_null())
        .filter(sys_menu::Column::Status.eq(1))
        .order_by(sys_menu::Column::OrderSort, Order::Asc)
        .into_model::<MenuResp>()
        .all(db)
        .await
        .map_err(BadRequest)?;
    Ok(s)
}
// pub async fn get_all_api<'a, C>(db: &'a C) -> Result<Vec<MenuResp>>
// where
//     C: TransactionTrait + ConnectionTrait,
// {
//     let menu_type = vec!["C", "F"];
//     let s = SysMenu::find()
//         .filter(sys_menu::Column::DeletedAt.is_null())
//         .filter(sys_menu::Column::Status.eq(1))
//         .filter(sys_menu::Column::MenuType.is_in(menu_type))
//         .order_by(sys_menu::Column::OrderSort, Order::Asc)
//         .into_model::<MenuResp>()
//         .all(db)
//         .await
//         .map_err(BadRequest)?;
//     Ok(s)
// }
/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_all_router(db: &DatabaseConnection) -> Result<Vec<MenuResp>> {
    let s = SysMenu::find()
        .filter(sys_menu::Column::DeletedAt.is_null())
        .filter(sys_menu::Column::Status.eq(1))
        .filter(sys_menu::Column::MenuType.ne(2))
        .order_by(sys_menu::Column::OrderSort, Order::Asc)
        .into_model::<MenuResp>()
        .all(db)
        .await
        .map_err(BadRequest)?;
    Ok(s)
}
/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_all_menu_tree(db: &DatabaseConnection) -> Result<Vec<SysMenuTree>> {
    let menus = get_all_router(db).await?;
    let menu_data = self::get_menu_data(menus);
    let menu_tree = self::get_menu_tree(menu_data, "0".to_string());

    Ok(menu_tree)
}

/// 获取授权菜单信息
pub async fn get_permissions(role_ids: Vec<String>) -> Vec<String> {
    let mut menu_apis: Vec<String> = Vec::new();
    let e = get_enforcer(false).await;
    for role_id in role_ids {
        let policies = e.get_filtered_policy(0, vec![role_id]);
        for policy in policies {
            menu_apis.push(policy[1].clone());
        }
    }
    menu_apis
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_admin_menu_by_role_ids(
    db: &DatabaseConnection,
    role_ids: Vec<String>,
) -> Result<Vec<SysMenuTree>> {
    let menu_apis = self::get_permissions(role_ids).await;
    //  todo 可能以后加条件判断
    let menu_all = get_all(db).await?;
    //  生成menus
    let mut menus: Vec<MenuResp> = Vec::new();
    for ele in menu_all {
        if menu_apis.contains(&ele.api) {
            menus.push(ele);
        }
    }
    let menu_data = self::get_menu_data(menus);
    let menu_tree = self::get_menu_tree(menu_data, "0".to_string());
    Ok(menu_tree)
}

pub fn get_menu_tree(user_menus: Vec<SysMenuTree>, pid: String) -> Vec<SysMenuTree> {
    let mut menu_tree: Vec<SysMenuTree> = Vec::new();
    for mut user_menu in user_menus.clone() {
        if user_menu.user_menu.pid == pid {
            user_menu.children = Some(get_menu_tree(
                user_menus.clone(),
                user_menu.user_menu.id.clone(),
            ));
            menu_tree.push(user_menu.clone());
        }
    }
    menu_tree
}

//  整理菜单数据 // todo
pub fn get_menu_data(menus: Vec<MenuResp>) -> Vec<SysMenuTree> {
    let mut menu_res: Vec<SysMenuTree> = Vec::new();
    for mut menu in menus {
        menu.pid = menu.pid.trim().to_string();
        let meta = Meta {
            icon: menu.icon.clone(),
            title: menu.menu_name.clone(),
            hidden: menu.visible.clone() != "1",
            link: if menu.path.clone().starts_with("http") {
                Some(menu.path.clone())
            } else {
                None
            },
            no_cache: menu.is_cache.clone() != "1",
        };
        let user_menu = UserMenu {
            meta,
            id: menu.id.clone(),
            pid: menu.pid.clone(),
            path: if !menu.path.clone().starts_with('/') && menu.pid.clone() == "0" {
                format!("/{}", menu.path.clone())
            } else {
                menu.path.clone()
            },
            name: menu.path.clone(),
            menu_name: menu.menu_name.clone(),
            menu_type: menu.menu_type.clone(),
            always_show: if menu.is_cache.clone() == "1" && menu.pid.clone() == "0" {
                Some(true)
            } else {
                None
            },
            component: menu.component.clone(),
            hidden: menu.visible.clone() == "0",
        };
        let menu_tree = SysMenuTree {
            user_menu,
            ..Default::default()
        };
        menu_res.push(menu_tree);
    }
    menu_res
}
