use anyhow::{anyhow, Result};
use db::{
    common::res::{ListData, PageParams},
    test::{
        entities::{prelude::TestDataScope, test_data_scope},
        models::test_data_scope::{AddReq, DeleteReq, SearchReq},
    },
};
use sea_orm::{ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, Set, TransactionTrait};

use crate::utils;

/// get_list 获取列表
/// page_params 分页参数
/// db 数据库连接 使用db.0
pub async fn get_sort_list(db: &DatabaseConnection, page_params: PageParams, search_req: SearchReq, user_id: &str) -> Result<ListData<test_data_scope::Model>> {
    let page_num = page_params.page_num.unwrap_or(1);
    let page_per_size = page_params.page_size.unwrap_or(10);
    //  生成查询条件
    let mut s = TestDataScope::find();

    if let Some(x) = search_req.data_a {
        s = s.filter(test_data_scope::Column::DataA.contains(&x));
    }

    if let Some(x) = search_req.data_b {
        s = s.filter(test_data_scope::Column::DataB.contains(&x));
    }
    let user_ids = utils::data_scope::get_data_scope_user_ids(db, user_id).await?;

    if let Some(x) = user_ids {
        if !x.is_empty() {
            s = s.filter(test_data_scope::Column::CreatedBy.is_in(x));
        }
    }

    // 获取全部数据条数
    let total = s.clone().count(db).await?;
    // 分页获取数据
    let paginator = s.order_by_asc(test_data_scope::Column::Id).paginate(db, page_per_size);
    let total_pages = paginator.num_pages().await?;
    let list = paginator.fetch_page(page_num - 1).await?;

    let res = ListData {
        total,
        list,
        total_pages,
        page_num,
    };
    Ok(res)
}

/// add 添加
pub async fn add<C>(db: &C, req: AddReq, user_id: &str) -> Result<String>
where
    C: TransactionTrait + ConnectionTrait,
{
    let add_data = test_data_scope::ActiveModel {
        id: Set(scru128::scru128_string()),
        data_a: Set(Some(req.data_a)),
        data_b: Set(Some(req.data_b)),
        created_by: Set(Some(user_id.to_string())),
    };
    TestDataScope::insert(add_data).exec(db).await?;
    Ok("添加成功".to_string())
}

/// delete 完全删除
pub async fn delete(db: &DatabaseConnection, delete_req: DeleteReq) -> Result<String> {
    let mut s = TestDataScope::delete_many();

    s = s.filter(test_data_scope::Column::Id.is_in(delete_req.ids));

    // 开始删除
    let d = s.exec(db).await?;

    match d.rows_affected {
        0 => Err(anyhow!("你要删除的数据不存在")),

        i => Ok(format!("成功删除{}条数据", i)),
    }
}
