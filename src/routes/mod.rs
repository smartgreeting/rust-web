/*
 * @Author: lihuan
 * @Date: 2025-03-21 22:36:06
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-23 21:20:15
 * @Email: 17719495105@163.com
 */

 use axum::Router;
  
 mod auth;
 mod user;
 
 pub fn init_router() -> Router {
  
    Router::new()
        .merge(auth::router())
        .merge(user::router())
        
}