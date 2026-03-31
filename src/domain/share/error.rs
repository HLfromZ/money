use crate::domain::share;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("参数错误: {0}")]
    ValidationError(String),

    #[error("{0}")]
    BusinessError(String),

    #[error("系统异常")]
    SystemError,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(msg) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                share::response::Response::<()>::fail_msg(&msg),
            )
                .into_response(),
            AppError::BusinessError(msg) => (
                StatusCode::OK,
                share::response::Response::<()>::fail_msg(&msg),
            )
                .into_response(),
            AppError::SystemError => (
                StatusCode::SERVICE_UNAVAILABLE,
                share::response::Response::<()>::fail_msg("系统异常"),
            )
                .into_response(),
        }
    }
}

impl From<validator::ValidationErrors> for AppError {
    fn from(errors: validator::ValidationErrors) -> Self {
        let mut msg = "参数错误: ".to_string();
        msg.push_str(
            &errors
                .field_errors()
                .iter()
                .map(|(field, errs)| {
                    let field_errors = errs
                        .iter()
                        .map(|err| err.to_string())
                        .collect::<Vec<_>>()
                        .join(", ");
                    format!("{}: {}", field, field_errors)
                })
                .collect::<Vec<_>>()
                .join(", "),
        );
        AppError::ValidationError(msg)
    }
}
