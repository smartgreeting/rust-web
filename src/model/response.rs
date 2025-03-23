use axum::Json;
use serde::Serialize;

#[derive(Serialize)]
pub struct ApiResponse<T> {
    code: u16,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 成功响应
    pub fn success(data: T) -> Json<Self> {
        Json(Self {
            code: 200,
            message: "success".to_string(),
            data: Some(data),
        })
    }

    /// 错误响应
    pub fn error(code: u16, message: &str) -> Json<Self> {
        Json(Self {
            code,
            message: message.to_string(),
            data: None,
        })
    }
}