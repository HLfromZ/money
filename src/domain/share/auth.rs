use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub user_id: i64,
    pub username: String,
    pub iat: usize,
    pub exp: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct AuthenticatedUser {
    pub user_id: i64,
    pub username: String,
}

impl From<Claims> for AuthenticatedUser {
    fn from(claims: Claims) -> Self {
        Self {
            user_id: claims.user_id,
            username: claims.username,
        }
    }
}
