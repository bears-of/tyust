use sqlx::{PgPool, Row};
use once_cell::sync::OnceCell;
use std::env;
use bcrypt::{hash, DEFAULT_COST};
use crate::entity::UserLoginInfo;
use crate::api_types::SemesterConfig;

static DB_POOL: OnceCell<PgPool> = OnceCell::new();

/// 初始化数据库连接池
pub async fn init_db_pool() -> Result<(), sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;
    DB_POOL.set(pool).map_err(|_| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, "Failed to set DB pool")))?;
    Ok(())
}

/// 获取数据库连接池
pub async fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().expect("Database pool not initialized")
}

/// 初始化数据库表
pub async fn init_tables(pool: &PgPool) -> Result<(), sqlx::Error> {
    // 创建用户表（添加avatar_url字段）
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            student_id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            class TEXT,
            token TEXT,
            avatar_url TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;
    
    // 如果表已存在但没有avatar_url字段，则添加该字段
    let _ = sqlx::query("ALTER TABLE users ADD COLUMN IF NOT EXISTS avatar_url TEXT")
        .execute(pool)
        .await;
    
    // 创建用户认证缓存表
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS user_auth_cache (
            student_id TEXT PRIMARY KEY,
            sourceid_tgc TEXT,
            rg_objectid TEXT,
            access_token TEXT,
            route TEXT,
            jwglxt_jsession TEXT,
            ronghemenhu_jsession TEXT,
            code TEXT,
            cached_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;
    
    // 创建登录统计表
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS login_stats (
            id SERIAL PRIMARY KEY,
            student_id TEXT NOT NULL,
            login_time TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;
    
    // 创建学期配置表
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS semester_config (
            id SERIAL PRIMARY KEY,
            semester_name TEXT NOT NULL,
            semester_start_date TEXT NOT NULL,
            is_active BOOLEAN DEFAULT true,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;
    
    // 创建管理员表
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS admins (
            id SERIAL PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )"
    )
    .execute(pool)
    .await?;
    
    // 创建默认管理员账户（如果不存在）
    // 使用环境变量中的默认密码，如果未设置则使用默认值
    let default_password = std::env::var("DEFAULT_ADMIN_PASSWORD")
        .unwrap_or_else(|_| "admin123".to_string());
    
    // 对密码进行哈希处理
    let hashed_password = bcrypt::hash(&default_password, bcrypt::DEFAULT_COST)
        .map_err(|e| sqlx::Error::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
    
    let _ = sqlx::query(
        "INSERT INTO admins (username, password_hash) 
         VALUES ($1, $2) 
         ON CONFLICT (username) DO NOTHING"
    )
    .bind("admin")
    .bind(&hashed_password)
    .execute(pool)
    .await;
    
    Ok(())
}

/// 保存用户信息（包括头像URL）
pub async fn save_user(pool: &PgPool, user: &UserLoginInfo) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (student_id, name, class, token, avatar_url, updated_at) 
         VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP)
         ON CONFLICT (student_id) 
         DO UPDATE SET name = EXCLUDED.name, class = EXCLUDED.class, token = EXCLUDED.token, avatar_url = EXCLUDED.avatar_url, updated_at = CURRENT_TIMESTAMP"
    )
    .bind(&user.student_id)
    .bind(&user.name)
    .bind(&user.class)
    .bind(&user.token)
    .bind(&user.avatar_url)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// 获取用户信息
pub async fn get_user(pool: &PgPool, student_id: &str) -> Result<Option<UserLoginInfo>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT student_id, name, class, token, avatar_url FROM users WHERE student_id = $1"
    )
    .bind(student_id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Some(UserLoginInfo {
            student_id: row.get(0),
            name: row.get(1),
            class: row.get(2),
            token: row.get(3),
            avatar_url: row.get(4),
        }))
    } else {
        Ok(None)
    }
}

/// 删除用户信息
pub async fn delete_user(pool: &PgPool, student_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM users WHERE student_id = $1")
        .bind(student_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// 保存用户认证缓存
pub async fn save_auth_cache(pool: &PgPool, student_id: &str, cache: &crate::auth::UserAuthCache) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO user_auth_cache (
            student_id, sourceid_tgc, rg_objectid, access_token, 
            route, jwglxt_jsession, ronghemenhu_jsession, code, cached_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
         ON CONFLICT (student_id) 
         DO UPDATE SET 
            sourceid_tgc = EXCLUDED.sourceid_tgc,
            rg_objectid = EXCLUDED.rg_objectid,
            access_token = EXCLUDED.access_token,
            route = EXCLUDED.route,
            jwglxt_jsession = EXCLUDED.jwglxt_jsession,
            ronghemenhu_jsession = EXCLUDED.ronghemenhu_jsession,
            code = EXCLUDED.code,
            cached_at = EXCLUDED.cached_at"
    )
    .bind(student_id)
    .bind(&cache.sourceid_tgc)
    .bind(&cache.rg_objectid)
    .bind(&cache.access_token)
    .bind(&cache.route)
    .bind(&cache.jwglxt_jsession)
    .bind(&cache.ronghemenhu_jsession)
    .bind(&cache.code)
    .bind(cache.cached_at.naive_utc())
    .execute(pool)
    .await?;
    
    Ok(())
}

/// 获取用户认证缓存
pub async fn get_auth_cache(pool: &PgPool, student_id: &str) -> Result<Option<crate::auth::UserAuthCache>, sqlx::Error> {
    let row = sqlx::query(
        "SELECT sourceid_tgc, rg_objectid, access_token, route, jwglxt_jsession, ronghemenhu_jsession, code, cached_at 
         FROM user_auth_cache WHERE student_id = $1"
    )
    .bind(student_id)
    .fetch_optional(pool)
    .await?;

    if let Some(row) = row {
        Ok(Some(crate::auth::UserAuthCache {
            sourceid_tgc: row.get(0),
            rg_objectid: row.get(1),
            access_token: row.get(2),
            route: row.get(3),
            jwglxt_jsession: row.get(4),
            ronghemenhu_jsession: row.get(5),
            code: row.get(6),
            cached_at: chrono::DateTime::from_naive_utc_and_offset(row.get(7), chrono::Utc),
        }))
    } else {
        Ok(None)
    }
}

/// 删除用户认证缓存
pub async fn delete_auth_cache(pool: &PgPool, student_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("DELETE FROM user_auth_cache WHERE student_id = $1")
        .bind(student_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// 清理过期的认证缓存（超过24小时）
pub async fn cleanup_expired_auth_cache(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let expired_time = chrono::Utc::now() - chrono::Duration::hours(24);
    
    let result = sqlx::query("DELETE FROM user_auth_cache WHERE cached_at < $1")
        .bind(expired_time.naive_utc())
        .execute(pool)
        .await?;
    
    Ok(result.rows_affected())
}

/// 记录登录统计
pub async fn record_login(pool: &PgPool, student_id: &str) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO login_stats (student_id, login_time) VALUES ($1, CURRENT_TIMESTAMP)")
        .bind(student_id)
        .execute(pool)
        .await?;
    
    Ok(())
}

/// 获取总登录次数
pub async fn get_total_logins(pool: &PgPool) -> Result<i64, sqlx::Error> {
    let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM login_stats")
        .fetch_one(pool)
        .await?;
    
    Ok(result.0)
}

/// 获取管理员信息
pub async fn get_admin(pool: &PgPool, username: &str) -> Result<Option<(i32, String, String)>, sqlx::Error> {
    let admin = sqlx::query_as::<_, (i32, String, String)>(
        "SELECT id, username, password_hash FROM admins WHERE username = $1"
    )
    .bind(username)
    .fetch_optional(pool)
    .await?;
    
    Ok(admin)
}

/// 创建管理员
pub async fn create_admin(pool: &PgPool, username: &str, password_hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO admins (username, password_hash) VALUES ($1, $2)"
    )
    .bind(username)
    .bind(password_hash)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// 更新管理员密码
pub async fn update_admin_password(pool: &PgPool, admin_id: i32, new_password_hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE admins SET password_hash = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
    )
    .bind(new_password_hash)
    .bind(admin_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// 更新管理员用户名
pub async fn update_admin_username(pool: &PgPool, admin_id: i32, new_username: &str) -> Result<(), sqlx::Error> {
    sqlx::query(
        "UPDATE admins SET username = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2"
    )
    .bind(new_username)
    .bind(admin_id)
    .execute(pool)
    .await?;
    
    Ok(())
}

/// 获取当前激活的学期配置
pub async fn get_active_semester_config(pool: &PgPool) -> Result<Option<SemesterConfig>, sqlx::Error> {
    let config = sqlx::query(
        "SELECT semester_name, semester_start_date 
         FROM semester_config 
         WHERE is_active = true 
         ORDER BY created_at DESC 
         LIMIT 1"
    )
    .fetch_optional(pool)
    .await?
    .map(|row| SemesterConfig {
        semester_name: row.get(0),
        semester_start_date: row.get(1),
    });
    
    Ok(config)
}

/// 保存或更新学期配置
pub async fn save_semester_config(pool: &PgPool, config: &SemesterConfig) -> Result<(), sqlx::Error> {
    // 先将所有配置设为非激活
    sqlx::query("UPDATE semester_config SET is_active = false")
        .execute(pool)
        .await?;
    
    // 插入新配置并设为激活
    sqlx::query(
        "INSERT INTO semester_config (semester_name, semester_start_date, is_active) 
         VALUES ($1, $2, true)"
    )
    .bind(&config.semester_name)
    .bind(&config.semester_start_date)
    .execute(pool)
    .await?;
    
    Ok(())
}