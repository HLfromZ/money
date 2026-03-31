use crate::domain::share::auth::AuthenticatedUser;
use crate::domain::share::error::AppError;
use crate::util::jwt;
use axum::extract::FromRequestParts;
use axum::http::header::AUTHORIZATION;
use axum::http::request::Parts;

impl<S> FromRequestParts<S> for AuthenticatedUser
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or_else(|| AppError::BusinessError("请先登录".to_string()))?;
        let auth_header = auth_header
            .to_str()
            .map_err(|_| AppError::BusinessError("请先登录".to_string()))?;
        let token = parse_bearer_token(auth_header)?;
        let claims = jwt::verify_access_token(token)?;

        Ok(claims.into())
    }
}

fn parse_bearer_token(auth_header: &str) -> Result<&str, AppError> {
    let (scheme, token) = auth_header
        .split_once(' ')
        .ok_or_else(|| AppError::BusinessError("请先登录".to_string()))?;

    if !scheme.eq_ignore_ascii_case("Bearer") {
        return Err(AppError::BusinessError("请先登录".to_string()));
    }

    let token = token.trim();
    if token.is_empty() {
        return Err(AppError::BusinessError("请先登录".to_string()));
    }

    Ok(token)
}
