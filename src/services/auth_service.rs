use crate::{database::session_dao, services::user_service};

pub async fn authenticate(username: String) -> anyhow::Result<AuthenticatedUser> {
    let user = user_service::get_user_by_username(username).await?;

    let session_opt = session_dao::get_session(user.id).await?;

    let session = match session_opt {
        Some(session) => session,
        None => {
            let token = uuid::Uuid::new_v4().to_string();
            session_dao::create_session(user.id, token).await?
        }
    };

    let authenticated_user = AuthenticatedUser {
        username: user.username,
        token: session.token,
    };

    return Ok(authenticated_user);
}

pub async fn _is_authenticated_user_id(token: String) -> anyhow::Result<Option<u32>> {
    let session_option = session_dao::_get_session_by_token(token).await?;

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
