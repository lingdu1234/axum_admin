// 创建一张表
use anyhow::{anyhow, Result};
use async_std::{
    fs::{self, File},
    io::{prelude::BufReadExt, BufReader},
    path::PathBuf,
    prelude::StreamExt,
};
pub use sea_orm::{ConnectionTrait, DatabaseConnection, DatabaseTransaction, Schema};
use sea_orm_migration::{
    prelude::*,
    sea_orm::{DatabaseBackend, EntityTrait, IdenStatic, Statement},
};

use crate::DATA_DIR;

/// 创建表格
///
/// e Entity
pub async fn creat_one_table<E>(db: &dyn ConnectionTrait, builder: DatabaseBackend, schema: &Schema, e: E) -> Result<(), DbErr>
where
    E: EntityTrait,
{
    match db.execute(builder.build(schema.create_table_from_entity(e).to_owned().if_not_exists())).await {
        Ok(_) => println!("创建表格成功:{}", e.table_name()),
        Err(e) => println!("{}", e),
    };

    Ok(())
}

///  创建表格索引
///
///  t:表格主题 `Entity`;
/// name:索引名称;
/// tp:索引类型 => u:unique,p:primary,i:index,f:fulltext;
pub async fn create_table_index<C, T>(manager: &SchemaManager<'_>, t: T, name: &str, cols: Vec<C>, tp: &str) -> Result<(), DbErr>
where
    C: 'static + IntoIndexColumn + IdenStatic + Clone + Copy,
    T: 'static + Iden + EntityTrait,
{
    let mut index = Index::create().name(name).table(t).to_owned();
    let mut cols_name = Vec::<String>::new();
    for co in cols {
        index = index.col(co).to_owned();
        cols_name.push(co.as_str().to_string());
    }
    match tp {
        "u" => {
            index = index.unique().to_owned();
        }
        "p" => {
            index = index.primary().to_owned();
        }
        "f" => {
            index = index.full_text().to_owned();
        }
        _ => {}
    }
    match manager.create_index(index.to_owned()).await {
        Ok(_) => println!("成功创建索引,表格:{},索引名:{},索引列:{:?}", t.table_name(), name, cols_name),
        Err(e) => println!("{}", e),
    };

    Ok(())
}

///  删除一张表
pub async fn drop_one_table<T>(manager: &SchemaManager<'_>, t: T) -> Result<(), DbErr>
where
    T: EntityTrait + IntoTableRef + 'static,
{
    manager.drop_table(Table::drop().table(t).to_owned()).await?;
    println!("成功删除表格:{}", t.table_name());
    Ok(())
}

/// 初始化数据
pub async fn init_data(manager: &SchemaManager<'_>, migration_name: &str) -> Result<(), DbErr> {
    let db = manager.get_connection();
    let db_end = manager.get_database_backend();
    let dir = DATA_DIR.to_owned() + migration_name;
    let mut entries = match fs::read_dir(&dir).await {
        Ok(x) => x,
        Err(_) => {
            println!("数据文件不存在，请先确认迁移文件是否存在");
            return Ok(());
        }
    };
    while let Some(res) = entries.next().await {
        let entry = match res {
            Ok(v) => v,
            Err(e) => {
                println!("{}", e);
                return Ok(());
            }
        };
        let path = entry.path();
        let sql_vec = match get_insert_sql_string(path.clone(), db_end).await {
            Ok(v) => v,
            Err(e) => {
                println!("{:?}", e);
                return Ok(());
            }
        };
        for sql in sql_vec {
            let stmt = Statement::from_string(db_end, sql).to_owned();
            match db.execute(stmt).await {
                Ok(_) => {
                    println!("表格数据初始化成功:{}", path.to_str().unwrap());
                }
                Err(e) => {
                    println!("{}", e);
                }
            };
        }
    }
    println!("全部表格数据初始化成功");
    Ok(())
}

async fn get_insert_sql_string(path: PathBuf, db_end: DatabaseBackend) -> Result<Vec<String>> {
    let mut sql: Vec<String> = Vec::new();
    let mut sql_string = String::new();
    let file = match File::open(path).await {
        Ok(x) => x,
        Err(e) => return Err(anyhow!("读取文件失败:{:?}", e.to_string())),
    };
    let mut buf_reader = BufReader::new(file).lines();
    while let Some(line) = buf_reader.next().await {
        match line {
            Err(e) => return Err(anyhow!("读取行数据失败:{:?}", e.to_string())),
            Ok(v) => {
                if v.starts_with("/*!") || v.starts_with("--") {
                    continue;
                }
                let vv = if v.starts_with("INSERT") || v.starts_with("insert") {
                    match db_end {
                        DatabaseBackend::MySql => v,
                        DatabaseBackend::Postgres => v.replace('`', "\""),
                        DatabaseBackend::Sqlite => v,
                    }
                } else {
                    v
                };

                sql_string.push_str(&vv);
                if vv.ends_with(';') {
                    sql.push(sql_string.clone());
                    sql_string.clear();
                }
            }
        }
    }

    Ok(sql)
}
