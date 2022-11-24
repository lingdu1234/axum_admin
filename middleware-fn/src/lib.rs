// 菜单授权
pub mod auth;
// 请求上下文，日志记录
pub mod ctx;

// 操作日志
pub mod oper_log;
// 缓存中间件
#[cfg(feature = "cache-mem")]
pub mod cache;

#[cfg(feature = "cache-skytable")]
pub mod cache_skytable;

//  重新导出
pub use auth::auth_fn_mid as ApiAuth;
#[cfg(feature = "cache-mem")]
pub use cache::cache_fn_mid as Cache;
#[cfg(feature = "cache-skytable")]
pub use cache_skytable::cache_fn_mid as SkyTableCache;
pub use ctx::ctx_fn_mid as Ctx;
pub use oper_log::oper_log_fn_mid as OperLog;
