use axum::{
    Router, middleware,
    routing::{get, post},
};
use tower_http::cors::CorsLayer;

mod api_types;
mod auth;
mod db;
mod de_crypto;
mod entity;
mod handlers;
mod http_helper;
mod tyust_api;
mod admin_handlers;

use auth::{auth_middleware, cleanup_expired_auth_cache};
use handlers::{get_schedule, get_user_info, init_semester_config, login, logout, get_courses, get_scores, get_raw_scores, init_login, get_login_code, get_semester_config};
use admin_handlers::{admin_login, get_students, get_semester, set_semester, get_statistics, update_admin_password, update_admin_username, admin_auth_middleware};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 加载 .env 文件
    dotenv::from_filename(".env").ok();
    
    // 初始化数据库连接池
    db::init_db_pool().await?;
    let db_pool = db::get_db_pool().await;
    println!("✅ Database initialized successfully");
    
    // 初始化数据库表
    db::init_tables(db_pool).await?;
    println!("✅ Database tables initialized successfully");
    
    // 清理过期的认证缓存
    if let Ok(count) = db::cleanup_expired_auth_cache(db_pool).await {
        if count > 0 {
            println!("🧹 Cleaned up {} expired auth cache entries", count);
        }
    }
    
    // 初始化学期配置
    if let Err(e) = init_semester_config().await {
        eprintln!("⚠️  Warning: Failed to load semester config: {}", e);
        println!("📝 Using default semester configuration");
    } else {
        println!("✅ Semester configuration loaded successfully");
    }
    
    // 创建需要认证的路由
    let protected_routes = Router::new()
        .route("/schedule", get(get_schedule))
        .route("/user/info", get(get_user_info))
        .route("/auth/logout", post(logout))
        .route("/scores", get(get_scores))
        .route("/raw-scores", get(get_raw_scores))
        .layer(middleware::from_fn(auth_middleware));

    // 创建管理员路由
    let admin_routes = Router::new()
        .route("/admin/students", get(get_students))
        .route("/admin/semester", get(get_semester))
        .route("/admin/semester", post(set_semester))
        .route("/admin/statistics", get(get_statistics))
        .route("/admin/password", post(update_admin_password))
        .route("/admin/username", post(update_admin_username))
        .layer(middleware::from_fn(admin_auth_middleware));

    // 创建管理员公开路由（不需要认证）
    let admin_public_routes = Router::new()
        .route("/admin/login", post(admin_login));

    // 创建API路由组
    let api_routes = Router::new()
        .route("/auth/login", post(login))
        .route("/login-init", get(init_login))
        .route("/login-code", get(get_login_code))
        .route("/courses", get(get_courses)) // 不使用中间件，自己处理认证
        .route("/semester-config", get(get_semester_config))
        .merge(protected_routes)
        .merge(admin_routes)
        .merge(admin_public_routes);

    // 创建应用
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(CorsLayer::permissive()) // 允许所有CORS请求
        .into_make_service();

    println!("🚀 Server starting on http://0.0.0.0:3000");
    println!("📚 Available endpoints:");
    println!("  POST /api/auth/login - 用户登录");
    println!("  GET  /api/login-init - 初始化验证码登录");
    println!("  GET  /api/login-code - 获取验证码图片");
    println!("  POST /api/login-verify - 验证码登录");
    println!("  GET  /api/schedule - 获取课表 (需要认证)");
    println!("  GET  /api/user/info - 获取用户信息 (需要认证)");
    println!("  POST /api/auth/logout - 用户登出 (需要认证)");
    println!("  GET  /api/courses - 获取课程列表 (需要认证)");
    println!("  GET  /api/scores - 获取有效成绩 (需要认证)");
    println!("  GET  /api/raw-scores - 获取原始成绩 (需要认证)");
    println!("");
    println!("  管理员接口:");
    println!("  POST /api/admin/login - 管理员登录");
    println!("  GET  /api/admin/students - 获取学生列表 (需要认证)");
    println!("  GET  /api/admin/semester - 获取学期配置 (需要认证)");
    println!("  POST /api/admin/semester - 设置学期配置 (需要认证)");
    println!("  GET  /api/admin/statistics - 获取统计信息 (需要认证)");
    println!("  POST /api/admin/password - 修改管理员密码 (需要认证)");
    println!("  POST /api/admin/username - 修改管理员用户名 (需要认证)");

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}