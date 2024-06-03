use crate::database::user_dao;

pub async fn create_user(
    name: String,
    username: String,
    pool: &sqlx::SqlitePool,
) -> anyhow::Result<User> {
    let user_table = user_dao::create_user(name, username, pool).await?;
    return Ok(User::from_user_table(user_table));
}

pub async fn get_user(id: u32, pool: &sqlx::SqlitePool) -> anyhow::Result<User> {
    let user_table = user_dao::get_user(id, pool).await?;
    return Ok(User::from_user_table(user_table));
}

pub async fn update_user(
    id: u32,
    name: String,
    username: String,
    pool: &sqlx::SqlitePool,
) -> anyhow::Result<User> {
    let user_table = user_dao::update_user(id, name, username, pool).await?;
    return Ok(User::from_user_table(user_table));
}

pub async fn delete_user(id: u32, pool: &sqlx::SqlitePool) -> anyhow::Result<()> {
    user_dao::delete_user(id, pool).await?;
    return Ok(());
}

pub struct User {
    pub id: u32,
    pub name: String,
    pub username: String,
}

impl User {
    fn from_user_table(user_table: user_dao::UserTable) -> Self {
        User {
            id: user_table.id,
            name: user_table.name,
            username: user_table.username,
        }
    }
}
