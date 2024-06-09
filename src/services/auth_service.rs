use crate::{
    database::{session_dao, Pool},
    services::user_service,
};

pub async fn authenticate(username: String, pool: &Pool) -> anyhow::Result<AuthenticatedUser> {
    let user = user_service::get_user_by_username(username, pool)
        .await?
        .ok_or(anyhow::anyhow!("No user found with that username"))?;

    let session_opt = session_dao::get_session(user.id, pool).await?;

    let session = match session_opt {
        Some(session) => session,
        None => {
            let token = uuid::Uuid::new_v4().to_string();
            session_dao::create_session(user.id, token, pool).await?
        }
    };

    let authenticated_user = AuthenticatedUser {
        username: user.username,
        token: session.token,
    };

    return Ok(authenticated_user);
}

pub async fn is_authenticated_user_id(token: String, pool: &Pool) -> anyhow::Result<Option<u32>> {
    let session_option = session_dao::get_session_by_token(token, pool).await?;

    let Some(session) = session_option else {
        return Ok(None);
    };

    let user_id = session.user_id;

    return Ok(Some(user_id));
}

#[derive(Debug)]
pub struct AuthenticatedUser {
    pub username: String,
    pub token: String,
}
