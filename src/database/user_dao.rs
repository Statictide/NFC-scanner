use super::db;

pub async fn create_user(name: String, username: String) -> sqlx::Result<UserTable> {
    let user: UserTable =
        sqlx::query_as("insert into user (name, username) values ($1, $2) returning *")
            .bind(name)
            .bind(username)
            .fetch_one(db::get_db().await)
            .await?;

    Ok(user)
}

pub async fn _get_user(id: u32) -> sqlx::Result<UserTable> {
    let user: UserTable = sqlx::query_as("select * from user where id = $1")
        .bind(id)
        .fetch_one(db::get_db().await)
        .await?;

    Ok(user)
}

pub async fn get_user_by_username(username: String) -> sqlx::Result<UserTable> {
    let user_option: UserTable = sqlx::query_as("select * from user where username = $1")
        .bind(username)
        .fetch_one(db::get_db().await)
        .await?;

    Ok(user_option)
}

pub async fn _update_user(id: u32, name: String, username: String) -> sqlx::Result<UserTable> {
    let user: UserTable =
        sqlx::query_as("update user set name = $1, username = $2 where id = $3 returning *")
            .bind(name)
            .bind(username)
            .bind(id)
            .fetch_one(db::get_db().await)
            .await?;

    Ok(user)
}

pub async fn _delete_user(id: u32) -> sqlx::Result<()> {
    sqlx::query("delete from user where id = $1")
        .bind(id)
        .execute(db::get_db().await)
        .await?;

    Ok(())
}

#[derive(sqlx::FromRow)]
pub struct UserTable {
    pub id: u32,
    pub name: String,
    pub username: String,
}
