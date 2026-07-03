use sea_orm::Statement;
use sea_orm_migration::prelude::*;

use crate::DATA_DIR;

/// Read all .sql files from a directory, sorted by filename, returning (filename, content)
pub fn read_sql_files(dir: &str) -> Vec<(String, String)> {
    let mut result = Vec::new();
    let entries = match std::fs::read_dir(dir) {
        Ok(e) => e,
        Err(e) => {
            println!("Failed to read directory {}: {}", dir, e);
            return result;
        }
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) != Some("sql") {
            continue;
        }
        let fname = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_string();
        let content = match std::fs::read_to_string(&path) {
            Ok(c) => c,
            Err(e) => {
                println!("Failed to read file {:?}: {}", path, e);
                continue;
            }
        };
        result.push((fname, content));
    }
    result.sort_by(|a, b| a.0.cmp(&b.0));
    result
}

/// Split SQL file content into individual statements by `;`, skipping comments and empty lines
pub fn split_sql_statements(content: &str) -> Vec<String> {
    let mut statements = Vec::new();
    let mut current = String::new();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with("--") || trimmed.starts_with("/*!") {
            continue;
        }
        current.push_str(line);
        current.push('\n');
        if trimmed.ends_with(';') {
            let stmt = current.trim().to_string();
            if !stmt.is_empty() {
                statements.push(stmt);
            }
            current.clear();
        }
    }
    let remaining = current.trim().to_string();
    if !remaining.is_empty() {
        statements.push(remaining);
    }
    statements
}

/// Execute all SQL files in a migration directory (with savepoint fault tolerance)
///
/// - `migration_name`: migration name, used to locate `migration/data/<name>/` directory
/// - `savepoint_prefix`: savepoint name prefix (must be globally unique)
/// - `log_title`: log title
pub async fn execute_sql_dir(
    manager: &SchemaManager<'_>,
    migration_name: &str,
    savepoint_prefix: &str,
    log_title: &str,
) -> Result<(), DbErr> {
    println!("========== {} start ==========", log_title);
    let db = manager.get_connection();
    let backend = manager.get_database_backend();
    let dir = format!("{}{}", DATA_DIR, migration_name);
    let sql_files = read_sql_files(&dir);
    println!("Found {} SQL file(s)", sql_files.len());

    let mut total_stmts = 0;
    let mut failed = 0;

    for (fname, content) in &sql_files {
        let statements = split_sql_statements(content);
        for (i, stmt) in statements.iter().enumerate() {
            let sp = format!("{}_up_{}", savepoint_prefix, i);
            let _ = db
                .execute(Statement::from_string(
                    backend,
                    format!("SAVEPOINT {}", sp),
                ))
                .await;
            let sea_stmt = Statement::from_string(backend, stmt.clone());
            match db.execute(sea_stmt).await {
                Ok(res) => {
                    let _ = db
                        .execute(Statement::from_string(
                            backend,
                            format!("RELEASE SAVEPOINT {}", sp),
                        ))
                        .await;
                    let affected = res.rows_affected();
                    println!("  [OK]   stmt#{} {} (affected: {})", i, fname, affected);
                }
                Err(e) => {
                    println!("  [FAIL] stmt#{} {} -> {}", i, fname, e);
                    let _ = db
                        .execute(Statement::from_string(
                            backend,
                            format!("ROLLBACK TO SAVEPOINT {}", sp),
                        ))
                        .await;
                    failed += 1;
                }
            }
            total_stmts += 1;
        }
    }

    println!(
        "========== {} done: {}/{} statements succeeded, {} failed ==========",
        log_title,
        total_stmts - failed,
        total_stmts,
        failed
    );
    Ok(())
}

/// Execute a list of raw SQL statements (with savepoint fault tolerance, for down() methods)
///
/// - `sqls`: SQL statement slice
/// - `savepoint_prefix`: savepoint name prefix
/// - `log_title`: log title
pub async fn execute_raw_sqls(
    manager: &SchemaManager<'_>,
    sqls: &[&str],
    savepoint_prefix: &str,
    log_title: &str,
) -> Result<(), DbErr> {
    println!("========== {} start ==========", log_title);
    let db = manager.get_connection();
    let backend = manager.get_database_backend();

    for (i, sql) in sqls.iter().enumerate() {
        let sp = format!("{}_dn_{}", savepoint_prefix, i);
        let _ = db
            .execute(Statement::from_string(
                backend,
                format!("SAVEPOINT {}", sp),
            ))
            .await;
        let stmt = Statement::from_string(backend, sql.to_owned());
        match db.execute(stmt).await {
            Ok(_) => {
                let _ = db
                    .execute(Statement::from_string(
                        backend,
                        format!("RELEASE SAVEPOINT {}", sp),
                    ))
                    .await;
            }
            Err(e) => {
                println!("  [FAIL] rollback#{} -> {}", i, e);
                let _ = db
                    .execute(Statement::from_string(
                        backend,
                        format!("ROLLBACK TO SAVEPOINT {}", sp),
                    ))
                    .await;
            }
        }
    }

    println!("========== {} done ==========", log_title);
    Ok(())
}
