// utils/constants/security_constants.rs
/// 验证码相关缓存前缀
pub mod verify_code {
    pub const REGISTER: &str = "AUTH:VERIFY_CODE:REGISTER:";
    pub const LOGIN: &str = "AUTH:VERIFY_CODE:LOGIN:";
    pub const UPDATE_PASSWORD: &str = "AUTH:VERIFY_CODE:UPDATE_PASSWORD:";
}

/// 安全密钥配置
pub mod security_key {
    pub const LOGIN: &str = "QsCdA/3d8CkxZ6k5c6eA61==";
    pub const REGISTER: &str = "7Fd2u4qF/3k0z6O1c9AeC7==";
}

/// 权限相关配置
pub mod permission {
    pub const USER_PERMS_CACHE_PREFIX: &str = "AUTH:USER_PERMS:";
    pub const BLACK_TOKEN_CACHE_PREFIX: &str = "AUTH:BLACK_TOKEN:";
}