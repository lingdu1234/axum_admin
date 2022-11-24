mod system;
mod test_fn;

use anyhow::{anyhow, Result};

/// 此处配置任务名称，用于前端添加测试名称，用于调用任务函数
pub async fn go_run_task(params: Option<String>, task_name: String) -> Result<String> {
    match task_name.as_str() {
        "test_a" => test_fn::test_a(),
        "test_b" => test_fn::test_b(params),
        "test_c" => test_fn::test_c(params),
        "check_user_online" => system::check_user_online().await,
        "update_api_info" => system::update_api_info().await,
        _ => Err(anyhow!("任务 {} 未找到", task_name)),
    }
}
