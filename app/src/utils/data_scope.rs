use anyhow::{anyhow, Result};
use db::system::entities::{sys_dept, sys_role, sys_role_dept, sys_user};
use sea_orm::{sea_query::Query, ColumnTrait, Condition, DatabaseConnection, EntityTrait, QueryFilter};

///  获取数据所对应的id，需要在数据库中添加created_by字段
pub async fn get_data_scope_user_ids(db: &DatabaseConnection, uid: &str) -> Result<Option<Vec<String>>> {
    // let s = sys_user::Entity::find()
    //     .select_only()
    //     .column(sys_user::Column::Id)
    //     .column(sys_user::Column::DeptId)
    //     .column(sys_user::Column::RoleId)
    //     .column(sys_role::Column::DataScope)
    //     .join_rev(
    //         JoinType::LeftJoin,
    //         sys_role::Entity::belongs_to(sys_user::Entity)
    //             .from(sys_role::Column::RoleId)
    //             .to(sys_user::Column::RoleId)
    //             .into(),
    //     )
    //     .filter(sys_user::Column::Id.eq(uid))
    //     .into_model::<DataScopeInfo>()
    //     .one(db)
    //     .await?;

    //     let (user_id, dept_id, role_id, data_scope) = match s {
    //         Some(x) => (x.id, x.dept_id, x.role_id, x.data_scope),
    //         None => return Err(anyhow!("用户不存在")),
    //     };

    // 先获取用户
    let u_s = sys_user::Entity::find().filter(sys_user::Column::Id.eq(uid)).one(db).await?;
    let (user_id, dept_id, role_id, data_scope) = match u_s {
        None => return Err(anyhow!("用户不存在")),
        Some(x) => {
            let r_s = sys_role::Entity::find().filter(sys_role::Column::RoleId.eq(x.role_id.clone())).one(db).await?;
            match r_s {
                Some(v) => (x.id, x.dept_id, x.role_id, v.data_scope),
                None => return Err(anyhow!("用户对应的角色不存在")),
            }
        }
    };

    // 数据范围
    //  1：全部数据权限
    //  2：自定数据权限
    //  3：本部门数据权限
    //  4：本部门及以下数据权限
    //  5：仅本人数据权限
    let res = match data_scope.as_str() {
        "1" => None,
        "2" => {
            let s = sys_user::Entity::find()
                .filter(
                    Condition::any().add(
                        sys_user::Column::DeptId.in_subquery(
                            Query::select()
                                .column(sys_role_dept::Column::DeptId)
                                .from(sys_role_dept::Entity)
                                .and_where(sys_role_dept::Column::RoleId.eq(role_id))
                                .to_owned(),
                        ),
                    ),
                )
                .all(db)
                .await?;
            Some(s.into_iter().map(|x| x.id).collect::<Vec<String>>())
        }
        "3" => {
            let s = sys_user::Entity::find().filter(sys_user::Column::DeptId.eq(dept_id.clone())).all(db).await?;
            Some(s.into_iter().map(|x| x.id).collect::<Vec<String>>())
        }
        "4" => {
            let s = sys_dept::Entity::find().all(db).await?;
            let mut ids: Vec<(String, String)> = Vec::new();
            s.iter().for_each(|x| ids.push((x.parent_id.clone(), x.dept_id.clone())));

            let mut dept_ids = find_sid_by_pid::<String>(ids, dept_id.clone());
            // 这里只获取了子id，需要将自己的id也加入
            dept_ids.push(dept_id.clone());

            let s = sys_user::Entity::find().filter(sys_user::Column::DeptId.is_in(dept_ids)).all(db).await?;
            Some(s.into_iter().map(|x| x.id).collect::<Vec<String>>())
        }
        _ => Some(vec![user_id]),
    };
    Ok(res)
}

// (T,T) 前面为pid 后面为id
fn find_sid_by_pid<T: PartialEq + Eq + Ord + Clone>(list: Vec<(T, T)>, pid: T) -> Vec<T> {
    let mut new_vec: Vec<T> = Vec::new();
    for (p, s) in list.clone() {
        if p == pid {
            new_vec.push(s.clone());
            let f_vec = find_sid_by_pid(list.clone(), s.clone());
            new_vec.extend(f_vec);
        }
    }
    new_vec
}
