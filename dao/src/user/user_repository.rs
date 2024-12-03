use sqlx::MySqlPool;
use domain::entity::user::user::SysUser;

pub async fn get_all_users(pool: &MySqlPool) -> Result<Vec<SysUser>, sqlx::Error> {
    let users = sqlx::query_as::<_, SysUser>("SELECT * FROM sys_user")
        .fetch_all(pool)
        .await?;

    Ok(users)
}