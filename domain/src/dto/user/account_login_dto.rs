use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AccountLoginDTO {
    pub username: Option<String>,           // 用户昵称（别名 姓名）

    pub password: Option<String>,       // 职位
}