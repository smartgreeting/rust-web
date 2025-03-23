/*
 * @Author: lihuan
 * @Date: 2025-03-21 22:59:36
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-23 21:23:37
 * @Email: 17719495105@163.com
 */
use tower::timeout::Timeout;
use std::time::Duration;

/// 服务熔断中间件
pub fn circuit_breaker() -> Timeout<tonic::transport::Channel> {
    Timeout::new(Duration::from_secs(5))
}