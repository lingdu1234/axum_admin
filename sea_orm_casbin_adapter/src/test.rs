#[cfg(test)]
#[test]
fn main_test() {
    let m = DefaultModel::from_file("config/rbac_model.conf")
        .await
        .unwrap();
    // mysql://root:lingdu515639@127.0.0.1:13306/wk_data
    // postgres://postgres:lingdu515639@127.0.0.1:25432/wk

    let mut opt =
        ConnectOptions::new("mysql://root:lingdu515639@127.0.0.1:13306/wk_data2".to_owned());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8));
    let db = Database::connect(opt).await.expect("数据库打开失败");

    let mut e = Enforcer::new(m, db).await.unwrap();
}
