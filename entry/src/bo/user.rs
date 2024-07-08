use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAdditionalInfoBo {
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterUserResponseBo {
    pub username: String,
    pub display_name: String,
    pub additional_info: UserAdditionalInfoBo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterUserRequestBo {
    pub username: String,
    pub display_name: String,
    pub password: String,
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserResponseBo {
    pub username: String,
    pub display_name: String,
    pub additional_info: UserAdditionalInfoBo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUserRequestBo {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AuthUserResponseBo {
    pub username: String,
    pub display_name: String,
    pub additional_info: UserAdditionalInfoBo,
    pub auth_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAuthTokenBo {
    pub username: String,
    pub exp: i64,
    pub additional_info: UserAdditionalInfoBo,
}
