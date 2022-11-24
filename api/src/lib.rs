// api 模块
mod system; // 系统模块
mod test; // 测试模块

//  路由模块
mod api_doc;
mod route;

//  重新导出
pub use api_doc::OpenApiDoc;
pub use route::api;
