use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UserAdditionalInfoBo {
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterUserResponseBo {
    pub user_name: String,
    pub display_name: String,
    pub additional_info: UserAdditionalInfoBo,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegisterUserRequestBo {
    pub user_name: String,
    pub display_name: String,
    pub password: String,
    pub labels: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GetUserResponseBo {
    pub user_name: String,
    pub display_name: String,
    pub additional_info: UserAdditionalInfoBo,
}
