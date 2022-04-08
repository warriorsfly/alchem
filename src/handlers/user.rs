use alchem_schema::{repository, source::User};
use alchem_utils::{pool::DatabaseConnection, validate::ValidatableJson, Error};

use axum::Json;
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
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct UserToken {
    pub token: String,
    pub account: User,
}

pub(crate) async fn signup(
    dbc: DatabaseConnection,
    ValidatableJson(entity): ValidatableJson<UserForm>,
) -> Result<Json<User>, Error> {
    let usr = repository::signup(dbc, &entity.name, &entity.password)?;

    Ok(Json(usr))
}
// pub async fn login(
//     system: Data<DoubleZeroSystem>,
//     entity: Json<LoginForm>,
// ) -> Result<Json<UserToken>, AlchemError> {
//     validate(&entity)?;
//     let ur =
//         block(move || repository::login(&system.pool, &entity.name, &entity.password)).await??;
//     let claims = Claims::new(ur.id);
//     let res = UserToken {
//         token: create_jwt(claims)?,
//         account: ur,
//     };
//     respond_json(res)
// }
