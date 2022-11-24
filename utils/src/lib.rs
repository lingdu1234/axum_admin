//! 一些与service 无关的 utils

pub mod cert;
pub mod data_scope;
pub mod my_env;
pub mod rand_utils;

pub use rand_utils::{encrypt_password, rand_s};
