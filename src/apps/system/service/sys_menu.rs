use chrono::{Local, NaiveDateTime};
use poem::{error::BadRequest, http::StatusCode, Error, Result};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, Set,
};
use sea_orm_casbin_adapter::casbin::MgmtApi;
use serde_json::json;

use crate::apps::system::models::RespData;
use crate::utils::{get_enforcer, CASBIN};

use super::super::entities::{prelude::*, sys_menu};
use super::super::models::{
    sys_menu::{AddReq, DeleteReq, EditReq, MenuResp, Meta, SearchReq, SysMenuTree, UserMenu},
    PageParams,
};

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
    let mut s = SysMenu::find();

    if let Some(x) = search_req.title {
        s = s.filter(sys_menu::Column::Title.contains(&x));
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
    let num_pages = paginator.num_pages().await.map_err(BadRequest)?;
    let list = paginator
        .fetch_page(page_num - 1)
        .await
        .map_err(BadRequest)?;

    let resp = json!({
            "list": list,
            "total": total,
            "total_pages": num_pages,
            "page_num": page_num,
    });
    Ok(RespData::with_data(resp))
}

pub async fn check_router_is_exist(route_path: String, db: &DatabaseConnection) -> Result<bool> {
    let s1 = SysMenu::find().filter(sys_menu::Column::Name.eq(route_path));
    let count1 = s1.count(db).await.map_err(BadRequest)?;
    Ok(count1 > 0)
}

pub async fn check_f_router_is_exist(
    front_route_path: String,
    db: &DatabaseConnection,
) -> Result<bool> {
    let s2 = SysMenu::find()
        .filter(sys_menu::Column::Path.eq(front_route_path))
        .filter(sys_menu::Column::MenuType.ne(2));
    let count2 = s2.count(db).await.map_err(BadRequest)?;
    Ok(count2 > 0)
}

/// add 添加
pub async fn add(db: &DatabaseConnection, add_req: AddReq) -> Result<RespData> {
    //  检查数据是否存在
    if check_router_is_exist(add_req.clone().name, db).await? {
        return Err(Error::from_string("数据已存在", StatusCode::BAD_REQUEST));
    }
    if let Some(x) = add_req.clone().path {
        if check_f_router_is_exist(x, db).await? {
            return Err(Error::from_string("数据已存在", StatusCode::BAD_REQUEST));
        }
    }

    let uid = scru128::scru128().to_string();
    let now: NaiveDateTime = Local::now().naive_local();
    let active_model = sys_menu::ActiveModel {
        id: Set(uid.clone()),
        pid: Set(add_req.pid),
        name: Set(add_req.name),
        title: Set(add_req.title),
        method: Set(add_req.method),
        icon: Set(add_req.icon.unwrap_or_else(|| "".to_string())),
        remark: Set(add_req.remark.unwrap_or_else(|| "".to_string())),
        menu_type: Set(add_req.menu_type),
        order_sort: Set(add_req.order_sort),
        status: Set(add_req.status),
        hidden: Set(add_req.hidden),
        keep_alive: Set(add_req.keep_alive),
        path: Set(add_req.path.unwrap_or_else(|| "".to_string())),
        jump_path: Set(add_req.jump_path.unwrap_or_else(|| "".to_string())),
        component: Set(add_req.component.unwrap_or_else(|| "".to_string())),
        allow_data_scope: Set(add_req.allow_data_scope),
        is_data_scope: Set(add_req.is_data_scope),
        is_frame: Set(add_req.is_frame),
        module_type: Set(add_req.module_type),
        model_id: Set(add_req.model_id),
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
    let resp = json!({ "id": uid });
    Ok(RespData::with_data(resp))
}

/// delete 完全删除
pub async fn ddelete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<RespData> {
    let mut s = SysMenu::delete_many();

    s = s.filter(sys_menu::Column::Id.is_in(delete_req.menu_ids));

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
    //  检查数据是否存在
    if check_router_is_exist(edit_req.clone().name, db).await? {
        return Err(Error::from_string("数据已存在", StatusCode::BAD_REQUEST));
    }
    if edit_req.clone().path != "" {
        if check_f_router_is_exist(edit_req.clone().path, db).await? {
            return Err(Error::from_string("数据已存在", StatusCode::BAD_REQUEST));
        }
    }

    let uid = edit_req.id;
    let s_s = SysMenu::find_by_id(uid.clone())
        .one(db)
        .await
        .map_err(BadRequest)?;
    let s_r: sys_menu::ActiveModel = s_s.unwrap().into();
    let now: NaiveDateTime = Local::now().naive_local();
    let act = sys_menu::ActiveModel {
        id: Set(uid.clone()),
        pid: Set(edit_req.pid),
        name: Set(edit_req.name),
        title: Set(edit_req.title),
        method: Set(edit_req.method),
        icon: Set(edit_req.icon),
        remark: Set(edit_req.remark),
        menu_type: Set(edit_req.menu_type),
        order_sort: Set(edit_req.order_sort),
        status: Set(edit_req.status),
        hidden: Set(edit_req.hidden),
        keep_alive: Set(edit_req.keep_alive),
        path: Set(edit_req.path),
        jump_path: Set(edit_req.jump_path),
        component: Set(edit_req.component),
        allow_data_scope: Set(edit_req.allow_data_scope),
        is_data_scope: Set(edit_req.is_data_scope),
        is_frame: Set(edit_req.is_frame),
        module_type: Set(edit_req.module_type),
        model_id: Set(edit_req.model_id),
        updated_at: Set(Some(now)),
        ..s_r
    };
    // 更新
    let _aa = act.update(db).await.map_err(BadRequest)?; //这个两种方式一样 都要多查询一次

    return Ok(RespData::with_msg(&format!("用户<{}>数据更新成功", uid)));
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
pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<MenuResp>> {
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

/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_all_menu_tree(db: &DatabaseConnection) -> Result<Vec<SysMenuTree>> {
    let menus = get_all(db).await?;
    let menu_data = self::get_menu_data(menus);
    let menu_tree = self::get_menu_tree(menu_data, "0".to_string());

    Ok(menu_tree)
}

/// 获取授权菜单信息
pub async fn get_permissions(role_ids: Vec<String>) -> Vec<String> {
    let e = CASBIN.get_or_init(get_enforcer).await.lock().await;
    let mut menu_ids: Vec<String> = Vec::new();
    for role_id in role_ids {
        let policies = e.get_filtered_policy(0, vec![role_id]);
        for policy in policies {
            menu_ids.push(policy[1].clone());
        }
    }
    menu_ids
}

/// get_all 获取全部   
/// db 数据库连接 使用db.0
pub async fn get_admin_menu_by_role_ids(
    db: &DatabaseConnection,
    role_ids: Vec<String>,
) -> Result<Vec<SysMenuTree>> {
    let menu_ids = self::get_permissions(role_ids).await;
    //  todo 可能以后加条件判断
    let menu_all = get_all(db).await?;
    //  生成menus
    let mut menus: Vec<MenuResp> = Vec::new();
    for ele in menu_all {
        if menu_ids.contains(&ele.id) {
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
        if user_menu.user_menu.menu.pid == pid {
            user_menu.children = Some(get_menu_tree(
                user_menus.clone(),
                user_menu.user_menu.menu.id.clone(),
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
            title: menu.title.clone(),
            keep_alive: menu.keep_alive.clone(),
            hidden: menu.hidden.clone(),
        };
        let user_menu = UserMenu { menu, meta };
        let menu_tree = SysMenuTree {
            user_menu,
            ..Default::default()
        };
        menu_res.push(menu_tree);
    }
    menu_res
}
