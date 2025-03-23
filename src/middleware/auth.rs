/*
 * @Author: lihuan
 * @Date: 2025-03-21 22:36:43
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-23 21:02:25
 * @Email: 17719495105@163.com
 */
use axum::{http::Request, middleware::Next, response::Response};
use crate::{error::HttpError, utils::jwt::JwtUtil};
/// JWT认证中间件
pub async fn jwt_auth<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, HttpError> {
    let token = req.headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or(HttpError::Unauthorized("Missing token".into()))?;

    let jwt_util = req.extensions()
        .get::<JwtUtil>()
        .expect("JwtUtil not initialized");
    
    let claims = jwt_util.validate_token(token)
        .map_err(|_| HttpError::Unauthorized("Invalid token".into()))?;

    // 将用户信息存入请求扩展
    req.extensions_mut().insert(claims.user_id);
    
    Ok(next.run(req).await)
}

/// 管理员权限检查
pub async fn admin_check<B>(
    req: Request<B>,
    next: Next<B>,
) -> Result<Response, HttpError> {
    let role = req.extensions()
        .get::<String>()
        .ok_or(HttpError::Forbidden)?;

    if role != "admin" {
        return Err(HttpError::Forbidden);
    }

    Ok(next.run(req).await)
}