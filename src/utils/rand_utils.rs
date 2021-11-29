use md5;
use rand::Rng;
///  生成指定长度的字符串
pub fn rand_s(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();

    let rand_string: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();

    rand_string
}

///  密码加密
pub fn encrypt_password(password: &str, salt: &str) -> String {
    use std::fmt::Write;
    let s = password.to_owned() + salt;
    let digest = md5::compute(s).to_vec();

    let mut result = String::new();
    for a in digest.iter() {
        write!(result, "{:02x}", a).unwrap();
    }
    result
}
