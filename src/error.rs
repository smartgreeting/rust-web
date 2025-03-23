/*
 * @Author: lihuan
 * @Date: 2025-03-23 20:31:30
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-23 20:32:12
 * @Email: 17719495105@163.com
 */

use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Authentication failed")]
    Unauthorized,
    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    // 添加其他错误类型...
}

pub type Result<T> = std::result::Result<T, AppError>;