use poem::handler;

use crate::CFG;

#[handler]
pub fn say_chacha() -> String {
    println!("~~~~~~~~~~~~~~~~{:?}", &CFG.jwt.jwt_secret);
    "我查, 我VHScdsd!".to_string()
}

#[handler]
pub fn say_chacha2() -> String {
    "我查, fgsdgsdgs!".to_string()
}
