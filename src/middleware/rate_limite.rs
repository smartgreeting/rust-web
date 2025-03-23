/*
 * @Author: lihuan
 * @Date: 2025-03-21 22:57:25
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-23 21:40:51
 * @Email: 17719495105@163.com
 */
use tower::Layer;
use std::time::Duration;
use crate::config::AppConfig;
// 修改导入路径为RateLimitLayer
use tower::limit::{RateLimitLayer, rate::Rate};

pub fn rate_limiter<S>(config: &AppConfig) -> impl Layer<S> {
    let rate = Rate::new(
        config.rate_limit.fill_rate,
        Duration::from_secs(1)
    );
    // 使用RateLimitLayer构造限流层
    RateLimitLayer::new(config.rate_limit.capacity, rate)
}