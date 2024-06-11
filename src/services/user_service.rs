use crate::database::user_dao;

pub async fn create_user(name: String, username: String) -> anyhow::Result<User> {
    let user_table = user_dao::create_user(name, username).await?;
    return Ok(User::from_user_table(user_table));
}

pub async fn get_user_by_username(username: String) -> anyhow::Result<Option<User>> {
    let user_table = user_dao::try_get_user_by_username(username).await?;
    let user = user_table.map(User::from_user_table);
    return Ok(user);
}

#[allow(dead_code)]
pub async fn get_user(id: u32) -> anyhow::Result<User> {
    let user_table = user_dao::get_user(id).await?;
    return Ok(User::from_user_table(user_table));
}

#[allow(dead_code)]
pub async fn update_user(id: u32, name: String, username: String) -> anyhow::Result<User> {
    let user_table = user_dao::update_user(id, name, username).await?;
    return Ok(User::from_user_table(user_table));
}

#[allow(dead_code)]
pub async fn delete_user(id: u32) -> anyhow::Result<()> {
    user_dao::delete_user(id).await?;
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
