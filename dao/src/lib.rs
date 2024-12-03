pub mod user;
mod db;

use db::get_db_pool;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::user::user_repository::get_all_users;

    // test get_all_users function from user_repository module in user module in dao crate
    #[tokio::test]
    async fn test_get_all_users() {
        let pool = get_db_pool().await.unwrap();
        let users = get_all_users(&pool).await.unwrap();
        // println the other user
        println!("{:?}", users[3]);
    }
}
