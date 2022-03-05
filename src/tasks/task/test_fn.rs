//!  所有任务的参数必须为 `params: Option<String`
//!
//! 如果任务没有参数，则可以不传, 也可以传 `None`
//!
//! 通过`serde_json::from_str`转换为对应的结构体
//!
//! 所有参数有对应的结构体类型，并实现`Deserialize`,或者为基本类型
//!
//! 返回值分别为 Result<T, anyhow::Error> T为string类型, 可以直接转换为json
//!  
//! 自动任务的返回值无法返回，只能作为任务记录

use anyhow::{anyhow, Result};
use serde::Deserialize;

/// 无参数测试
pub fn test_a() -> Result<String> {
    println!("无参数函数测试");
    Ok("test_a函数运行成功".to_string())
}

/// 简单参数测试
pub fn test_b(params: Option<String>) -> Result<String> {
    let param = match params {
        Some(x) => x,
        None => {
            println!("参数为空");
            return Err(anyhow!("参数为空, 请检查参数"));
        }
    };
    println!("-----------test_b-----------参数为: {}", param);
    let b_string: String = param;
    println!("简单参数函数测试，参数为：{}", b_string);
    println!("{}", b_string);
    Ok("test_b函数运行成功".to_string())
}

#[derive(Deserialize, Debug)]
struct TestC {
    a: String,
}
/// JSON字符串参数测试
pub fn test_c(params: Option<String>) -> Result<String> {
    let param = match params {
        Some(x) => x,
        None => {
            println!("参数为空");
            return Err(anyhow!("参数为空, 请检查参数"));
        }
    };
    println!("-----------test_c-----------参数为: {}", param);
    let pp: TestC = match serde_json::from_str(param.as_str()) {
        Ok(x) => x,
        Err(e) => {
            println!("参数解析失败:{}", e);
            return Err(anyhow!("参数解析失败,请检查参数格式,参数为{}", param));
        }
    };
    println!("简单参数函数测试,参数为: {:#?},a的值{}", pp, pp.a);
    let res = format!("test_c函数运行成功,参数为:{:#?}", pp);
    Ok(res)
}
