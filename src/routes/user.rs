/*
 * @Author: lihuan
 * @Date: 2025-03-21 23:06:34
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-21 23:11:47
 * @Email: 17719495105@163.com
 */
use axum::Router;
use axum::routing::get;


pub fn router() -> Router {
    Router::new().route("/user", get(gethandler))
      
}

async  fn gethandler() -> &'static str {
    "user"
}