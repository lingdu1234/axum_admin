use anyhow::{anyhow, Result};
use chrono::Local;
use db::{db_conn, system::SysUserOnlineEntity, DB};
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::service_utils;

//  检查在线用户任
pub async fn check_user_online() -> Result<String> {
    let s = check_online_auto_task().await;
    let v = match s {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("{}", e.to_string())),
    };

    Ok(v)
}

//  检查用户是否在线,不在线清除
async fn check_online_auto_task() -> Result<String> {
    let mut n: i64 = 0;
    let db = DB.get_or_init(db_conn).await;
    let s = SysUserOnlineEntity::Entity::find().all(db).await.map_err(|e| anyhow!("{}", e.to_string()))?;
    for item in s {
        let now = Local::now();
        if item.token_exp < now.timestamp() {
            SysUserOnlineEntity::Entity::delete_many()
                .filter(SysUserOnlineEntity::Column::TokenId.eq(item.token_id))
                .exec(db)
                .await
                .map_err(|e| anyhow!("{}", e.to_string()))?;
            n += 1;
        }
    }
    Ok(format!("清除 {} 位在线过期用户", n))
}

/// 更新全局api信息
pub async fn update_api_info() -> Result<String> {
    service_utils::api_utils::re_init_all_api().await;
    Ok("api信息更新成功".to_string())
}
