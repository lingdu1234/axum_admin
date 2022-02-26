pub fn setup() {
    //   打印logo
    self::show_log();
}

fn show_log() {
    let logo = r#"    
                                           | |         (_)
     _ __   ___   ___ _ __ ___     __ _  __| |_ __ ___  _ _ __
    | '_ \ / _ \ / _ \ '_ ` _ \   / _` |/ _` | '_ ` _ \| | '_ \
    | |_) | (_) |  __/ | | | | | | (_| | (_| | | | | | | | | | |
    | .__/ \___/ \___|_| |_| |_|  \__,_|\__,_|_| |_| |_|_|_| |_|
    | |
    |_|
       "#;
    println!("{}", logo);
    // println!("系统架构：{}", std::env::var("OS").unwrap().to_string());
    // println!("系统类型：{}", std::env::consts::ARCH);
    // println!("操作系统：{}", std::env::consts::FAMILY);
    // println!()
}
