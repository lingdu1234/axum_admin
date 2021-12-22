use poem::{delete, get, post, Route};
mod sys_dict_data;
mod sys_dict_type;
mod sys_post;
mod sys_role;
mod sys_user;

pub fn system_api() -> Route {
    Route::new()
        .at("/login", post(sys_user::login)) //登录
        .nest("/user", sys_user_api()) //用户管理模块
        .nest("/dict_type", sys_dict_type_api()) //字典类型模块
        .nest("/dict_data", sys_dict_data_api()) //字典数据模块
        .nest("/post", sys_post_api()) //岗位模块
        .nest("/role", sys_role_api()) //角色模块
}

fn sys_user_api() -> Route {
    Route::new()
        .at("/get", get(sys_user::get_sort_list)) //获取全部用户
        .at("/get_by_id", get(sys_user::get_by_id_or_name)) //按id获取用户
        .at("/add", post(sys_user::add)) //添加用户
        .at("/edit", post(sys_user::edit)) //更新用户
        .at("/delete", delete(sys_user::delete)) //软删除用户
        .at("/ddelete", delete(sys_user::ddelete)) //硬删除用户
}

fn sys_dict_type_api() -> Route {
    Route::new()
        .at("/get", get(sys_dict_type::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_dict_type::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_dict_type::get_by_id)) //按id获取
        .at("/add", post(sys_dict_type::add)) //添加
        .at("/edit", post(sys_dict_type::edit)) //更新
        // .at("/delete", delete(sys_dict_type::delete)) //软删除
        .at("/ddelete", delete(sys_dict_type::ddelete)) //硬删除
}

fn sys_dict_data_api() -> Route {
    Route::new()
        .at("/get", get(sys_dict_data::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_dict_data::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_dict_data::get_by_id)) //按id获取
        .at("/add", post(sys_dict_data::add)) //添加
        .at("/edit", post(sys_dict_data::edit)) //更新
        // .at("/delete", delete(sys_dict_data::delete)) //软删除
        .at("/ddelete", delete(sys_dict_data::ddelete)) //硬删除
}

fn sys_post_api() -> Route {
    Route::new()
        .at("/get", get(sys_post::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_post::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_post::get_by_id)) //按id获取
        .at("/add", post(sys_post::add)) //添加
        .at("/edit", post(sys_post::edit)) //更新
        // .at("/delete", delete(sys_post::delete)) //软删除
        .at("/ddelete", delete(sys_post::ddelete)) //硬删除
}

fn sys_role_api() -> Route {
    Route::new()
        .at("/get", get(sys_role::get_sort_list)) //获取筛选分页
        .at("/get_all", get(sys_role::get_all)) //获取筛选分页
        .at("/get_by_id", get(sys_role::get_by_id)) //按id获取
        .at("/add", post(sys_role::add)) //添加
        .at("/edit", post(sys_role::edit)) //更新
        // .at("/delete", delete(sys_role::delete)) //软删除
        .at("/ddelete", delete(sys_role::ddelete)) //硬删除
}
