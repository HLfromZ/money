use crate::error_sys;
use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;

pub static OK_CODE: i32 = 0;
pub static FAIL_CODE: i32 = -1;

#[derive(Debug, Serialize)]
pub struct Response<T> {
    status_code: u16,
    body: Body<T>,
}

#[derive(Debug, Serialize)]
struct Body<T> {
    pub code: i32,
    pub data: Option<T>,
    pub msg: String,
}

impl<T> Response<T>
where
    T: Serialize,
{
    pub fn ok_msg(msg: &str) -> Self {
        Self {
            status_code: 200,
            body: Body {
                code: OK_CODE,
                data: None,
                msg: msg.to_string(),
            },
        }
    }
    pub fn ok_data(data: T) -> Self {
        Self {
            status_code: 200,
            body: Body {
                code: OK_CODE,
                data: Some(data),
                msg: "".to_string(),
            },
        }
    }
    pub fn ok_data_msg(data: T, msg: &str) -> Self {
        Self {
            status_code: 200,
            body: Body {
                code: OK_CODE,
                data: Some(data),
                msg: msg.to_string(),
            },
        }
    }
    pub fn fail_msg(msg: &str) -> Self {
        Self {
            status_code: 200,
            body: Body {
                code: FAIL_CODE,
                data: None,
                msg: msg.to_string(),
            },
        }
    }
    pub fn fail_data(data: T) -> Self {
        Self {
            status_code: 200,
            body: Body {
                code: FAIL_CODE,
                data: Some(data),
                msg: "".to_string(),
            },
        }
    }
    pub fn fail_data_msg(data: T, msg: &str) -> Self {
        Self {
            status_code: 200,
            body: Body {
                code: FAIL_CODE,
                data: Some(data),
                msg: msg.to_string(),
            },
        }
    }
    pub fn create_msg(msg: &str) -> Self {
        Self {
            status_code: 201,
            body: Body {
                code: OK_CODE,
                data: None,
                msg: msg.to_string(),
            },
        }
    }
    pub fn create_data(data: T) -> Self {
        Self {
            status_code: 201,
            body: Body {
                code: OK_CODE,
                data: Some(data),
                msg: "".to_string(),
            },
        }
    }
    pub fn create_data_msg(data: T, msg: &str) -> Self {
        Self {
            status_code: 201,
            body: Body {
                code: OK_CODE,
                data: Some(data),
                msg: msg.to_string(),
            },
        }
    }
}

impl<T> IntoResponse for Body<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        Json(self).into_response()
    }
}

impl<T> IntoResponse for Response<T>
where
    T: Serialize,
{
    fn into_response(self) -> axum::response::Response {
        let status_code = StatusCode::from_u16(self.status_code).unwrap_or_else(|_| {
            error_sys!("错误的 StatusCode");
            StatusCode::INTERNAL_SERVER_ERROR
        });
        (status_code, self.body).into_response()
    }
}
