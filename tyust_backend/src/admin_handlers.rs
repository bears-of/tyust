use axum::{
    Json,
    extract::{Extension},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::{
    api_types::{ApiResponse},
    db,
};

// 管理员登录请求参数
#[derive(Deserialize)]
pub struct AdminLoginRequest {
    pub username: String,
    pub password: String,
}

// 管理员登录响应
#[derive(Serialize)]
pub struct AdminLoginResponse {
    pub token: String,
    pub username: String,
}

// 修改管理员密码请求
#[derive(Deserialize)]
pub struct UpdateAdminPasswordRequest {
    pub old_password: String,
    pub new_password: String,
}

// 修改管理员用户名请求
#[derive(Deserialize)]
pub struct UpdateAdminUsernameRequest {
    pub new_username: String,
    pub password: String,
}

// 管理员登录接口
pub async fn admin_login(
    Json(params): Json<AdminLoginRequest>,
) -> Result<Json<ApiResponse<AdminLoginResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let db_pool = db::get_db_pool().await;
    
    // 从数据库获取管理员信息
    match db::get_admin(db_pool, &params.username).await {
        Ok(Some((admin_id, username, password_hash))) => {
            // 验证密码（使用 bcrypt 哈希算法）
            match verify(&params.password, &password_hash) {
                Ok(true) => {
                    // 生成 JWT token
                    use jsonwebtoken::{encode, Header, EncodingKey};
                    use crate::api_types::AdminClaims;
                    
                    let claims = AdminClaims {
                        sub: admin_id.to_string(),
                        username: username.clone(),
                        exp: chrono::Utc::now().timestamp() as usize + 3600, // 1小时过期
                        iat: chrono::Utc::now().timestamp() as usize,
                    };
                    
                    // 使用环境变量中的密钥
                    let jwt_secret = std::env::var("ADMIN_JWT_SECRET")
                        .unwrap_or_else(|_| "fallback_admin_secret_key".to_string());
                    
                    let token = match encode(&Header::default(), &claims, &EncodingKey::from_secret(jwt_secret.as_ref())) {
                        Ok(t) => t,
                        Err(_) => {
                            return Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ApiResponse::error(500, "Failed to generate token".to_string())),
                            ));
                        }
                    };
                    
                    let response = AdminLoginResponse {
                        token,
                        username: username,
                    };
                    
                    Ok(Json(ApiResponse::success(response)))
                },
                Ok(false) | Err(_) => {
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(ApiResponse::error(401, "用户名或密码错误".to_string())),
                    ))
                }
            }
        },
        Ok(None) => {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(401, "用户名或密码错误".to_string())),
            ))
        },
        Err(_) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(500, "数据库查询失败".to_string())),
            ))
        }
    }
}

// 获取学生列表
pub async fn get_students(
    Extension(_admin_id): Extension<String>,
) -> Result<Json<ApiResponse<Vec<crate::entity::UserLoginInfo>>>, (StatusCode, Json<ApiResponse<()>>)> {
    let db_pool = db::get_db_pool().await;
    
    // 查询所有用户
    let users = sqlx::query("SELECT student_id, name, class, token FROM users")
        .fetch_all(db_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch users: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(500, "获取学生列表失败".to_string())),
            )
        })?;
    
    let students: Vec<crate::entity::UserLoginInfo> = users
        .into_iter()
        .map(|row| crate::entity::UserLoginInfo {
            student_id: row.get::<&str, _>(0).to_string(),
            name: row.get::<&str, _>(1).to_string(),
            class: row.get::<Option<&str>, _>(2).map(|s| s.to_string()).unwrap_or_default(),
            token: row.get::<Option<&str>, _>(3).map(|s| s.to_string()).unwrap_or_default(),
        })
        .collect();
    
    Ok(Json(ApiResponse::success(students)))
}

// 获取学期配置
pub async fn get_semester(
    Extension(_admin_id): Extension<String>,
) -> Result<Json<ApiResponse<crate::api_types::SemesterConfig>>, (StatusCode, Json<ApiResponse<()>>)> {
    let db_pool = db::get_db_pool().await;
    
    let config = db::get_active_semester_config(db_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch semester config: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(500, "获取学期配置失败".to_string())),
            )
        })?;
    
    match config {
        Some(config) => Ok(Json(ApiResponse::success(config))),
        None => Err((
            StatusCode::NOT_FOUND,
            Json(ApiResponse::error(404, "学期配置未设置".to_string())),
        )),
    }
}

// 设置学期配置
#[derive(Deserialize)]
pub struct SetSemesterRequest {
    pub semester_name: String,
    pub start_date: String,
}

pub async fn set_semester(
    Extension(_admin_id): Extension<String>,
    Json(params): Json<SetSemesterRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let db_pool = db::get_db_pool().await;
    
    let config = crate::api_types::SemesterConfig {
        semester_name: params.semester_name,
        semester_start_date: params.start_date,
    };
    
    db::save_semester_config(db_pool, &config)
        .await
        .map_err(|e| {
            eprintln!("Failed to save semester config: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(500, "保存学期配置失败".to_string())),
            )
        })?;
    
    Ok(Json(ApiResponse::success(())))
}

// 获取统计信息
#[derive(Serialize)]
pub struct StatisticsResponse {
    pub total_students: i64,
    pub active_students: i64,
    pub total_logins: i64,
}

pub async fn get_statistics(
    Extension(_admin_id): Extension<String>,
) -> Result<Json<ApiResponse<StatisticsResponse>>, (StatusCode, Json<ApiResponse<()>>)> {
    let db_pool = db::get_db_pool().await;
    
    // 获取总学生数
    let total_students: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(db_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch total students: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(500, "获取统计信息失败".to_string())),
            )
        })?;
    
    // 获取活跃学生数（最近24小时内登录的）
    let cutoff_time = chrono::Utc::now() - chrono::Duration::hours(24);
    let cutoff_time_str = cutoff_time.format("%Y-%m-%d %H:%M:%S").to_string();
    let active_students: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE updated_at > $1::timestamp"
    )
    .bind(&cutoff_time_str)
    .fetch_one(db_pool)
    .await
    .map_err(|e| {
        eprintln!("Failed to fetch active students: {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(500, "获取统计信息失败".to_string())),
        )
    })?;
    
    // 获取总登录次数
    let total_logins = db::get_total_logins(db_pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to fetch total logins: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(500, "获取统计信息失败".to_string())),
            )
        })?;
    
    let stats = StatisticsResponse {
        total_students: total_students.0,
        active_students: active_students.0,
        total_logins,
    };
    
    Ok(Json(ApiResponse::success(stats)))
}

// 更新管理员密码
pub async fn update_admin_password(
    Extension(admin_username): Extension<String>,
    Json(params): Json<UpdateAdminPasswordRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let db_pool = db::get_db_pool().await;
    
    // 获取管理员信息
    match db::get_admin(db_pool, &admin_username).await {
        Ok(Some((admin_id, _, password_hash))) => {
            // 验证旧密码
            match verify(&params.old_password, &password_hash) {
                Ok(true) => {
                    // 对新密码进行哈希处理
                    match hash(&params.new_password, DEFAULT_COST) {
                        Ok(hashed_password) => {
                            // 更新密码
                            match db::update_admin_password(db_pool, admin_id, &hashed_password).await {
                                Ok(()) => Ok(Json(ApiResponse::success(()))),
                                Err(_) => Err((
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(ApiResponse::error(500, "更新密码失败".to_string())),
                                )),
                            }
                        },
                        Err(_) => Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::error(500, "密码加密失败".to_string())),
                        )),
                    }
                },
                Ok(false) | Err(_) => {
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(ApiResponse::error(401, "旧密码错误".to_string())),
                    ))
                }
            }
        },
        Ok(None) => {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(401, "管理员不存在".to_string())),
            ))
        },
        Err(_) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(500, "数据库查询失败".to_string())),
            ))
        }
    }
}

// 更新管理员用户名
pub async fn update_admin_username(
    Extension(admin_username): Extension<String>,
    Json(params): Json<UpdateAdminUsernameRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let db_pool = db::get_db_pool().await;
    
    // 获取管理员信息
    match db::get_admin(db_pool, &admin_username).await {
        Ok(Some((admin_id, _, password_hash))) => {
            // 验证密码
            match verify(&params.password, &password_hash) {
                Ok(true) => {
                    // 检查新用户名是否已存在
                    match db::get_admin(db_pool, &params.new_username).await {
                        Ok(Some(_)) => {
                            // 用户名已存在
                            Err((
                                StatusCode::CONFLICT,
                                Json(ApiResponse::error(409, "用户名已存在".to_string())),
                            ))
                        },
                        Ok(None) => {
                            // 用户名可用，更新用户名
                            match db::update_admin_username(db_pool, admin_id, &params.new_username).await {
                                Ok(()) => Ok(Json(ApiResponse::success(()))),
                                Err(_) => Err((
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    Json(ApiResponse::error(500, "更新用户名失败".to_string())),
                                )),
                            }
                        },
                        Err(_) => {
                            Err((
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ApiResponse::error(500, "数据库查询失败".to_string())),
                            ))
                        }
                    }
                },
                Ok(false) | Err(_) => {
                    Err((
                        StatusCode::UNAUTHORIZED,
                        Json(ApiResponse::error(401, "密码错误".to_string())),
                    ))
                }
            }
        },
        Ok(None) => {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(401, "管理员不存在".to_string())),
            ))
        },
        Err(_) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiResponse::error(500, "数据库查询失败".to_string())),
            ))
        }
    }
}

// 管理员认证中间件
pub async fn admin_auth_middleware(
    headers: axum::http::HeaderMap,
    mut request: axum::extract::Request,
    next: axum::middleware::Next,
) -> Result<axum::response::Response, (StatusCode, Json<ApiResponse<()>>)> {
    // 从Authorization头获取token
    let auth_header = headers
        .get("authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| {
            if header.starts_with("Bearer ") {
                Some(&header[7..])
            } else {
                None
            }
        });

    let token = match auth_header {
        Some(token) => token,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(401, "缺少认证token".to_string())),
            ));
        }
    };

    // 验证JWT token
    use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
    use crate::api_types::AdminClaims;
    
    // 使用环境变量中的密钥
    let jwt_secret = std::env::var("ADMIN_JWT_SECRET")
        .unwrap_or_else(|_| "fallback_admin_secret_key".to_string());
    
    match decode::<AdminClaims>(&token, &DecodingKey::from_secret(jwt_secret.as_ref()), &Validation::new(Algorithm::HS256)) {
        Ok(token_data) => {
            // 将管理员用户名添加到请求扩展中
            request.extensions_mut().insert(token_data.claims.username);
            Ok(next.run(request).await)
        },
        Err(_) => {
            Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(401, "无效的认证token".to_string())),
            ))
        }
    }
}