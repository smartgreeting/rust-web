/*
 * @Author: lihuan
 * @Date: 2025-03-21 22:37:40
 * @LastEditors: lihuan
 * @LastEditTime: 2025-03-21 22:58:21
 * @Email: 17719495105@163.com
 */
use sqlx::{PgPool, FromRow};
use crate::model::user::User;

#[derive(Debug, Clone)]
pub struct UserDao {
    pool: PgPool,
}

impl UserDao {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// 根据ID查询用户
    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash, role, created_at, updated_at 
            FROM users 
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
    }

    /// 创建新用户
    pub async fn create_user(
        &self,
        username: &str,
        password_hash: &str,
        role: &str
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (username, password_hash, role)
            VALUES ($1, $2, $3)
            RETURNING *
            "#,
            username,
            password_hash,
            role
        )
        .fetch_one(&self.pool)
        .await
    }
}