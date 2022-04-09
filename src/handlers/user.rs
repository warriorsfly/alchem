use alchem_schema::{repo, source::User};
use alchem_utils::{
    claims::Armor,
    config::{CONFIG, KEY_PAIR},
    db::DatabaseConnection,
    validate::ValidatedJson,
    Error,
};

use axum::Json;
use jwt_simple::prelude::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct UserForm {
    pub name: String,
    #[validate(phone(message = "phone must be a valid phone number"))]
    pub phone: String,
    #[validate(length(min = 4))]
    pub password: String,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct PatchUserForm {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub bio: Option<String>,
    pub avatar: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct LoginForm {
    pub name: String,
    #[validate(length(min = 4))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserToken {
    pub token: String,
    pub account: User,
}

pub(crate) async fn signup_handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    ValidatedJson(entity): ValidatedJson<UserForm>,
) -> Result<Json<User>, Error> {
    let usr = repo::signup(
        &mut conn,
        entity.name.to_owned(),
        entity.password.to_owned(),
    )
    .await?;

    Ok(Json(usr))
}
pub(crate) async fn login_handler(
    DatabaseConnection(mut conn): DatabaseConnection,
    ValidatedJson(entity): ValidatedJson<LoginForm>,
) -> Result<Json<UserToken>, Error> {
    let usr = repo::login(
        &mut conn,
        entity.name.to_owned(),
        entity.password.to_owned(),
    )
    .await?;
    let armor = Armor { id: usr.id };
    let claims = Claims::with_custom_claims(armor, Duration::from_secs(CONFIG.jwt_expire_seconds));
    let token = KEY_PAIR
        .sign(claims)
        .map_err(|e| Error::InternalServerError(e.to_string()))?;
    Ok(Json(UserToken {
        token,
        account: usr,
    }))
}
