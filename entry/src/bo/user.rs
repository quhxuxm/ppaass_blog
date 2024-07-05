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
