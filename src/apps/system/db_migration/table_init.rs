use crate::utils;

use super::super::models::{
    sys_dict_data::AddReq as SysDictDataAddReq, sys_dict_type::AddReq as SysDictTypeAddReq,
};
use super::super::{
    entities::{
        sys_dept::ActiveModel as SysDeptActiveModel, sys_menu::ActiveModel as SysMenuActiveModel, *,
    },
    service,
};
use chrono::{Local, NaiveDateTime};
use scru128::scru128_string;
use sea_orm::{ActiveModelTrait, EntityTrait, QueryOrder, Set, TransactionTrait};
pub use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, Schema};

pub async fn database_init(db: &DatabaseConnection) {
    // 1. 查看数据库版本
    let s = sys_db_version::Entity::find()
        .order_by_desc(sys_db_version::Column::UpdatedAt)
        .one(db)
        .await;
    match s {
        Ok(Some(x)) => match x.db_version {
            1 => {
                //  后续版本更新 数据库迁移
            }
            2 => {
                //  后续版本更新 数据库迁移
            }
            _ => {}
        },
        Ok(None) => {}
        Err(_) => {
            // 当无法获取数据库数据版本号时，认为无数据，初始化数据库
            println!("数据库版本不存在，开始数据库初始化");
            db_table_init(db).await;
            db_table_data_init(db).await;
        }
    }
}

/// 创建用户表
async fn db_table_init(db: &DatabaseConnection) {
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    // 创建表
    db.execute(builder.build(&schema.create_table_from_entity(sys_db_version::Entity)))
        .await
        .expect("数据库创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_user::Entity)))
        .await
        .expect("数据库创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_dept::Entity)))
        .await
        .expect("数据库创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_dict_type::Entity)))
        .await
        .expect("数据库创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_dict_data::Entity)))
        .await
        .expect("数据库创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_menu::Entity)))
        .await
        .expect("数据库创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_post::Entity)))
        .await
        .expect("数据库创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_user_post::Entity)))
        .await
        .expect("用户部门表,创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_role::Entity)))
        .await
        .expect("角色表,创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_role_dept::Entity)))
        .await
        .expect("角色部门表,创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_login_log::Entity)))
        .await
        .expect("登录日志表,创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_user_online::Entity)))
        .await
        .expect("在线用户表,创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_job::Entity)))
        .await
        .expect("定时任务表,创建失败或者已经存在");
    db.execute(builder.build(&schema.create_table_from_entity(sys_job_log::Entity)))
        .await
        .expect("定时任务日志表,创建失败或者已经存在");
}

async fn db_table_data_init(db: &DatabaseConnection) {
    let now: NaiveDateTime = Local::now().naive_local();
    let txn = db.begin().await.expect("数据库事务开启失败");
    let uid = "00TV87DDOBJPU75J4TGUOC3NNG";
    let dept_id = &scru128_string();
    let post_id = &scru128_string();
    let role_id = &scru128_string();
    //  初始化数据库版本数据
    casbin_rule_init(uid, role_id).await;
    sys_db_version_init(&txn, now).await;
    sys_user_init(&txn, now, uid, dept_id).await;
    sys_dept_init(&txn, now, uid, dept_id).await;
    sys_post_init(&txn, now, uid, post_id).await;
    sys_role_init(&txn, now, uid, role_id).await;
    sys_role_dept_init(&txn, now, dept_id, role_id).await;
    sys_user_post_init(&txn, now, uid, post_id).await;
    sys_dict_type_init(&txn, uid).await;
    sys_dict_data_init(&txn, uid).await;
    sys_menu_init(&txn).await;
    txn.commit().await.expect("数据库事务提交失败");
}

async fn casbin_rule_init(uid: &str, role_id: &str) {
    let role_ids = vec![role_id.to_string()];
    service::sys_role::add_role_by_user_id(uid, role_ids)
        .await
        .expect("添加用户角色失败");
}

async fn sys_db_version_init(txn: &DatabaseTransaction, now: NaiveDateTime) {
    let init_data = sys_db_version::ActiveModel {
        id: Set(scru128::scru128_string()),
        db_version: Set(1),
        updated_at: Set(now),
    };
    init_data
        .insert(txn)
        .await
        .expect("sys_update_info_init_data insert error");
}

async fn sys_user_init(txn: &DatabaseTransaction, now: NaiveDateTime, uid: &str, dept_id: &str) {
    let salt = utils::rand_s(10);
    let passwd = utils::encrypt_password("123456", &salt);
    let init_data = sys_user::ActiveModel {
        id: Set(uid.to_string()),
        user_name: Set("admin".to_string()),
        user_nickname: Set("超级管理员".to_string()),
        user_password: Set(passwd),
        user_salt: Set(salt),
        user_status: Set("1".to_string()),
        user_email: Set("admin@admin.com".to_string()),
        sex: Set("0".to_string()),
        avatar: Set("".to_string()),
        dept_id: Set(dept_id.to_string()),
        remark: Set("".to_string()),
        is_admin: Set("1".to_string()),
        phone_num: Set("18888888888".to_string()),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    init_data
        .insert(txn)
        .await
        .expect("sys_user_init_data insert error");
}

async fn sys_dept_init(txn: &DatabaseTransaction, now: NaiveDateTime, uid: &str, dept_id: &str) {
    let init_data: Vec<SysDeptActiveModel> = vec![
        SysDeptActiveModel {
            dept_id: Set(dept_id.to_string()),
            parent_id: Set("0".to_string()),
            dept_name: Set("lingdu".to_string()),
            order_num: Set(1000),
            leader: Set("admin".to_string()),
            phone: Set("18888888888".to_string()),
            email: Set("admin@admin.com".to_string()),
            status: Set("1".to_string()),
            created_by: Set(uid.to_string()),
            created_at: Set(Some(now)),
            ..Default::default()
        },
        SysDeptActiveModel {
            dept_id: Set(scru128_string()),
            parent_id: Set(dept_id.to_string()),
            dept_name: Set("部门A".to_string()),
            order_num: Set(1100),
            leader: Set("A".to_string()),
            phone: Set("18888888888".to_string()),
            email: Set("A@admin.com".to_string()),
            status: Set("1".to_string()),
            created_by: Set(uid.to_string()),
            created_at: Set(Some(now)),
            ..Default::default()
        },
        SysDeptActiveModel {
            dept_id: Set(scru128_string()),
            parent_id: Set(dept_id.to_string()),
            dept_name: Set("部门A".to_string()),
            order_num: Set(1200),
            leader: Set("B".to_string()),
            phone: Set("18888888888".to_string()),
            email: Set("B@admin.com".to_string()),
            status: Set("1".to_string()),
            created_by: Set(uid.to_string()),
            created_at: Set(Some(now)),
            ..Default::default()
        },
    ];
    sys_dept::Entity::insert_many(init_data)
        .exec(txn)
        .await
        .expect("sys_dept_init_data insert error");
}

async fn sys_post_init(txn: &DatabaseTransaction, now: NaiveDateTime, uid: &str, post_id: &str) {
    let init_data = sys_post::ActiveModel {
        post_id: Set(post_id.to_string()),
        post_code: Set("CEO".to_string()),
        post_name: Set("董事长".to_string()),
        post_sort: Set(1),
        status: Set("1".to_string()),
        remark: Set(Some("董事长".to_string())),
        created_by: Set(uid.to_string()),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    init_data
        .insert(txn)
        .await
        .expect("sys_post_init_data insert error");
}

async fn sys_role_init(txn: &DatabaseTransaction, now: NaiveDateTime, _uid: &str, role_id: &str) {
    let init_data = sys_role::ActiveModel {
        role_id: Set(role_id.to_string()),
        role_name: Set("超级管理员".to_string()),
        role_key: Set("SuperAdmin".to_string()),
        list_order: Set(0),
        data_scope: Set("1".to_string()),
        status: Set("1".to_string()),
        remark: Set("超级管理员".to_string()),
        created_at: Set(Some(now)),
        ..Default::default()
    };
    init_data
        .insert(txn)
        .await
        .expect("sys_role_init_data insert error");
}

async fn sys_role_dept_init(
    txn: &DatabaseTransaction,
    now: NaiveDateTime,
    dept_id: &str,
    role_id: &str,
) {
    let init_data = sys_role_dept::ActiveModel {
        role_id: Set(role_id.to_string()),
        dept_id: Set(dept_id.to_string()),
        created_at: Set(Some(now)),
    };
    init_data
        .insert(txn)
        .await
        .expect("sys_role_dept_init_data insert error");
}

async fn sys_user_post_init(
    txn: &DatabaseTransaction,
    now: NaiveDateTime,
    uid: &str,
    post_id: &str,
) {
    let init_data = sys_user_post::ActiveModel {
        user_id: Set(uid.to_string()),
        post_id: Set(post_id.to_string()),
        created_at: Set(Some(now)),
    };
    init_data
        .insert(txn)
        .await
        .expect("sys_user_post_init_data insert error");
}

async fn sys_dict_type_init(txn: &DatabaseTransaction, uid: &str) {
    let u_id = uid.to_string();
    let sys_dept_types: Vec<SysDictTypeAddReq> = vec![
        SysDictTypeAddReq {
            dict_name: "用户性别".to_string(),
            dict_type: "sys_user_sex".to_string(),
            status: Some("1".to_string()),
            remark: Some("用户性别".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "菜单状态".to_string(),
            dict_type: "sys_show_hide".to_string(),
            status: Some("1".to_string()),
            remark: Some("菜单状态".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "系统开关".to_string(),
            dict_type: "sys_normal_disable".to_string(),
            status: Some("1".to_string()),
            remark: Some("系统开关".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "任务状态".to_string(),
            dict_type: "sys_job_status".to_string(),
            status: Some("1".to_string()),
            remark: Some("任务状态".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "任务分组".to_string(),
            dict_type: "sys_job_group".to_string(),
            status: Some("1".to_string()),
            remark: Some("任务分组".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "系统是否".to_string(),
            dict_type: "sys_yes_no".to_string(),
            status: Some("1".to_string()),
            remark: Some("系统是否".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "通知类型".to_string(),
            dict_type: "sys_notice_type".to_string(),
            status: Some("1".to_string()),
            remark: Some("通知类型".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "通知状态".to_string(),
            dict_type: "sys_notice_status".to_string(),
            status: Some("1".to_string()),
            remark: Some("通知状态".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "操作类型".to_string(),
            dict_type: "sys_oper_type".to_string(),
            status: Some("1".to_string()),
            remark: Some("操作类型".to_string()),
        },
        SysDictTypeAddReq {
            dict_name: "系统状态".to_string(),
            dict_type: "sys_common_status".to_string(),
            status: Some("1".to_string()),
            remark: Some("系统状态".to_string()),
        },
    ];
    for ele in sys_dept_types {
        service::sys_dict_type::add(txn, ele, u_id.clone())
            .await
            .expect("sys_dict_type_init insert error");
    }
}

async fn sys_dict_data_init(txn: &DatabaseTransaction, uid: &str) {
    let u_id = uid.to_string();
    let sys_dept_types: Vec<SysDictDataAddReq> = vec![
        SysDictDataAddReq {
            dict_type: "sys_user_sex".to_string(),
            dict_label: "未知".to_string(),
            dict_value: "0".to_string(),
            dict_sort: 0,
            is_default: "N".to_string(),
            list_class: Some("default".to_string()),
            remark: Some("性别神秘".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_user_sex".to_string(),
            dict_label: "男".to_string(),
            dict_value: "1".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("default".to_string()),
            remark: Some("性别男".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_user_sex".to_string(),
            dict_label: "女".to_string(),
            dict_value: "2".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("default".to_string()),
            remark: Some("性别女".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_show_hide".to_string(),
            dict_label: "显示".to_string(),
            dict_value: "1".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("primary".to_string()),
            remark: Some("显示菜单".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_show_hide".to_string(),
            dict_label: "隐藏".to_string(),
            dict_value: "0".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("danger".to_string()),
            remark: Some("隐藏菜单".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_normal_disable".to_string(),
            dict_label: "正常".to_string(),
            dict_value: "1".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("primary".to_string()),
            remark: Some("正常状态".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_normal_disable".to_string(),
            dict_label: "停用".to_string(),
            dict_value: "0".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("danger".to_string()),
            remark: Some("停用状态".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_job_status".to_string(),
            dict_label: "正常".to_string(),
            dict_value: "1".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("primary".to_string()),
            remark: Some("正常状态".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_job_status".to_string(),
            dict_label: "暂停".to_string(),
            dict_value: "0".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("warning".to_string()),
            remark: Some("暂停状态".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_job_group".to_string(),
            dict_label: "默认".to_string(),
            dict_value: "DEFAULT".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("default".to_string()),
            remark: Some("默认分组".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_job_group".to_string(),
            dict_label: "系统".to_string(),
            dict_value: "SYSTEM".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("default".to_string()),
            remark: Some("系统分组".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_yes_no".to_string(),
            dict_label: "是".to_string(),
            dict_value: "Y".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("primary".to_string()),
            remark: Some("系统默认是".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_yes_no".to_string(),
            dict_label: "否".to_string(),
            dict_value: "N".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("danger".to_string()),
            remark: Some("系统默认否".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_notice_type".to_string(),
            dict_label: "通知".to_string(),
            dict_value: "1".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("warning".to_string()),
            remark: Some("通知".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_notice_type".to_string(),
            dict_label: "公告".to_string(),
            dict_value: "2".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("success".to_string()),
            remark: Some("公告".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_notice_status".to_string(),
            dict_label: "正常".to_string(),
            dict_value: "1".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("primary".to_string()),
            remark: Some("正常状态".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_notice_status".to_string(),
            dict_label: "关闭".to_string(),
            dict_value: "0".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("danger".to_string()),
            remark: Some("关闭状态".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_common_status".to_string(),
            dict_label: "成功".to_string(),
            dict_value: "1".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("primary".to_string()),
            remark: Some("成功状态".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_common_status".to_string(),
            dict_label: "失败".to_string(),
            dict_value: "0".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("danger".to_string()),
            remark: Some("失败状态".to_string()),
            ..Default::default()
        },
        // 操作状态
        SysDictDataAddReq {
            dict_type: "sys_oper_type".to_string(),
            dict_label: "新增".to_string(),
            dict_value: "1".to_string(),
            dict_sort: 1,
            is_default: "N".to_string(),
            list_class: Some("info".to_string()),
            remark: Some("新增操作".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_oper_type".to_string(),
            dict_label: "修改".to_string(),
            dict_value: "2".to_string(),
            dict_sort: 2,
            is_default: "N".to_string(),
            list_class: Some("info".to_string()),
            remark: Some("修改操作".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_oper_type".to_string(),
            dict_label: "删除".to_string(),
            dict_value: "3".to_string(),
            dict_sort: 3,
            is_default: "N".to_string(),
            list_class: Some("danger".to_string()),
            remark: Some("删除操作".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_oper_type".to_string(),
            dict_label: "授权".to_string(),
            dict_value: "4".to_string(),
            dict_sort: 4,
            is_default: "N".to_string(),
            list_class: Some("primary".to_string()),
            remark: Some("授权操作".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_oper_type".to_string(),
            dict_label: "导出".to_string(),
            dict_value: "5".to_string(),
            dict_sort: 5,
            is_default: "N".to_string(),
            list_class: Some("warning".to_string()),
            remark: Some("导出操作".to_string()),
            ..Default::default()
        },
        SysDictDataAddReq {
            dict_type: "sys_oper_type".to_string(),
            dict_label: "导入".to_string(),
            dict_value: "6".to_string(),
            dict_sort: 6,
            is_default: "N".to_string(),
            list_class: Some("warning".to_string()),
            remark: Some("导入操作".to_string()),
            ..Default::default()
        },
    ];
    for ele in sys_dept_types {
        service::sys_dict_data::add(txn, ele, u_id.clone())
            .await
            .expect("sys_dict_type_init insert error");
    }
}

async fn sys_menu_init(txn: &DatabaseTransaction) {
    let sys_menus: Vec<SysMenuActiveModel> = vec![
        SysMenuActiveModel {
            id: Set("00UDGE80NBMO4ST8TJMECI0A73".to_string()),
            pid: Set("0".to_string()),
            path: Set("system".to_string()),
            menu_name: Set("系统管理".to_string()),
            icon: Set("system".to_string()),
            menu_type: Set("M".to_string()),
            order_sort: Set(10000),
            status: Set("1".to_string()),
            api: Set("".to_string()),
            component: Set("".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("系统管理".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
        SysMenuActiveModel {
            id: Set("00UDGAK2ESG0UUFHII2UV2JCTD".to_string()),
            pid: Set("00UDGE80NBMO4ST8TJMECI0A73".to_string()),
            path: Set("basic".to_string()),
            menu_name: Set("基础数据".to_string()),
            icon: Set("table".to_string()),
            menu_type: Set("M".to_string()),
            order_sort: Set(11000),
            status: Set("1".to_string()),
            api: Set("".to_string()),
            component: Set("".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("基础数据".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
        SysMenuActiveModel {
            id: Set("00UDF5ES6C4NFHC07PM9GCVTEB".to_string()),
            pid: Set("00UDGE80NBMO4ST8TJMECI0A73".to_string()),
            path: Set("auth".to_string()),
            menu_name: Set("用户数据".to_string()),
            icon: Set("people".to_string()),
            menu_type: Set("M".to_string()),
            order_sort: Set(12000),
            status: Set("1".to_string()),
            api: Set("".to_string()),
            component: Set("".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("用户数据".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
        SysMenuActiveModel {
            id: Set("00UDGE80NBMO4SV8TJMF9J8DM3".to_string()),
            pid: Set("00UDGAK2ESG0UUFHII2UV2JCTD".to_string()),
            path: Set("menu".to_string()),
            menu_name: Set("菜单管理".to_string()),
            icon: Set("tree-table".to_string()),
            menu_type: Set("C".to_string()),
            order_sort: Set(11100),
            status: Set("1".to_string()),
            api: Set("system:menu:list".to_string()),
            component: Set("system/menu/index".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("菜单管理".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
        SysMenuActiveModel {
            id: Set("00TV8J02HNJMAUDJOV5F02BV35".to_string()),
            pid: Set("00UDGAK2ESG0UUFHII2UV2JCTD".to_string()),
            path: Set("dict".to_string()),
            menu_name: Set("字典管理".to_string()),
            icon: Set("dict".to_string()),
            menu_type: Set("C".to_string()),
            order_sort: Set(11200),
            status: Set("1".to_string()),
            api: Set("system:dict:list".to_string()),
            component: Set("system/dict/index".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("字典管理".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
        SysMenuActiveModel {
            id: Set("00TV8ENLC2DL61K03MS9LT04F3".to_string()),
            pid: Set("00UDF5ES6C4NFHC07PM9GCVTEB".to_string()),
            path: Set("user".to_string()),
            menu_name: Set("用户管理".to_string()),
            icon: Set("user".to_string()),
            menu_type: Set("C".to_string()),
            order_sort: Set(12100),
            status: Set("1".to_string()),
            api: Set("system:user:list".to_string()),
            component: Set("system/user/index".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("用户管理".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
        SysMenuActiveModel {
            id: Set("00TV8H55BU243CFAFK9O9GKL5P".to_string()),
            pid: Set("00UDF5ES6C4NFHC07PM9GCVTEB".to_string()),
            path: Set("dept".to_string()),
            menu_name: Set("部门管理".to_string()),
            icon: Set("tree".to_string()),
            menu_type: Set("C".to_string()),
            order_sort: Set(12200),
            status: Set("1".to_string()),
            api: Set("system:dept:list".to_string()),
            component: Set("system/dept/index".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("部门管理".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
        SysMenuActiveModel {
            id: Set("00TV8HV6G76KBTOJ74AM5BPFS1".to_string()),
            pid: Set("00UDF5ES6C4NFHC07PM9GCVTEB".to_string()),
            path: Set("post".to_string()),
            menu_name: Set("部门管理".to_string()),
            icon: Set("post".to_string()),
            menu_type: Set("C".to_string()),
            order_sort: Set(12300),
            status: Set("1".to_string()),
            api: Set("system:post:list".to_string()),
            component: Set("system/post/index".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("部门管理".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
        SysMenuActiveModel {
            id: Set("00TV8FT34RNV9T5PUSHHOVF56R".to_string()),
            pid: Set("00UDF5ES6C4NFHC07PM9GCVTEB".to_string()),
            path: Set("role".to_string()),
            menu_name: Set("角色管理".to_string()),
            icon: Set("peoples".to_string()),
            menu_type: Set("C".to_string()),
            order_sort: Set(12400),
            status: Set("1".to_string()),
            api: Set("system:role:list".to_string()),
            component: Set("system/role/index".to_string()),
            visible: Set("1".to_string()),
            is_cache: Set("1".to_string()),
            is_frame: Set("0".to_string()),
            is_data_scope: Set("0".to_string()),
            remark: Set("角色管理".to_string()),
            created_at: Set(Some(Local::now().naive_local())),
            ..Default::default()
        },
    ];
    sys_menu::Entity::insert_many(sys_menus)
        .exec(txn)
        .await
        .expect("sys_menu_init insert error");
}
