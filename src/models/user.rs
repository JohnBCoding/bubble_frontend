use crate::prelude::*;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UserLogin {
    pub email: String,
    pub password: String,
    #[serde(rename = "returnSecureToken")]
    pub return_secure_token: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
}
