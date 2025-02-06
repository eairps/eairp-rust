use sqlx::{query_as, MySqlPool};
use domain::entity::user::user::SysUser;
use crate::db::get_db_pool;

pub async fn find_user_by_credentials(username: &str, password: &str) -> Result<Option<SysUser>, sqlx::Error> {
    // let pool = get_db_pool().await?;
    let pool = MySqlPool::connect("mysql://root:123456@localhost:3306/eairp").await?;
    let user = query_as::<_, SysUser>("SELECT * FROM sys_user WHERE user_name = ? and password = ?")
        .bind(username)
        .bind(password)
        .fetch_optional(&pool)
        .await?;

    println!("pool: {:?}", pool);
    // 打印user
    println!("user: {:?}", user);

    Ok(user)
}