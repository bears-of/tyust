use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::api_types::{ApiResponse, Claims, UserInfo};

/// JWT密钥
static JWT_SECRET: &str = "tyust_course_system_secret_key_2024";

/// 用户认证缓存信息
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserAuthCache {
    pub sourceid_tgc: String,
    pub rg_objectid: String,
    pub access_token: String,
    pub route: String,
    pub jwglxt_jsession: String,
    pub ronghemenhu_jsession: String,
    pub code: String,
    pub cached_at: chrono::DateTime<chrono::Utc>,
}

/// 用户会话存储（简单内存存储，生产环境应使用Redis等）
static USER_SESSIONS: Lazy<Arc<Mutex<HashMap<String, UserInfo>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// 用户认证缓存存储
static USER_AUTH_CACHE: Lazy<Arc<Mutex<HashMap<String, UserAuthCache>>>> = 
    Lazy::new(|| Arc::new(Mutex::new(HashMap::new())));

/// 生成JWT Token
pub fn generate_token(student_id: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = chrono::Utc::now();
    let exp = (now + chrono::Duration::hours(24)).timestamp() as usize;
    let iat = now.timestamp() as usize;

    let claims = Claims {
        sub: student_id.to_string(),
        exp,
        iat,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

/// 验证JWT Token
pub fn verify_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
}

/// 存储用户会话
pub fn store_user_session(student_id: String, user_info: UserInfo) {
    let mut sessions = USER_SESSIONS.lock().unwrap();
    sessions.insert(student_id, user_info);
}

/// 获取用户会话
pub fn get_user_session(student_id: &str) -> Option<UserInfo> {
    let sessions = USER_SESSIONS.lock().unwrap();
    sessions.get(student_id).cloned()
}

/// 删除用户会话
pub fn remove_user_session(student_id: &str) {
    let mut sessions = USER_SESSIONS.lock().unwrap();
    sessions.remove(student_id);
}

/// 存储用户认证缓存
pub fn store_user_auth_cache(student_id: String, auth_cache: UserAuthCache) {
    let mut cache = USER_AUTH_CACHE.lock().unwrap();
    cache.insert(student_id, auth_cache);
}

/// 获取用户认证缓存
pub fn get_user_auth_cache(student_id: &str) -> Option<UserAuthCache> {
    let cache = USER_AUTH_CACHE.lock().unwrap();
    cache.get(student_id).cloned()
}

/// 删除用户认证缓存
pub fn remove_user_auth_cache(student_id: &str) {
    let mut cache = USER_AUTH_CACHE.lock().unwrap();
    cache.remove(student_id);
}

/// 检查认证缓存是否有效（24小时内）
pub fn is_auth_cache_valid(auth_cache: &UserAuthCache) -> bool {
    let now = chrono::Utc::now();
    let duration = now.signed_duration_since(auth_cache.cached_at);
    duration.num_hours() < 24
}

/// 清理过期的认证缓存
pub fn cleanup_expired_auth_cache() {
    let mut cache = USER_AUTH_CACHE.lock().unwrap();
    let now = chrono::Utc::now();
    cache.retain(|_, auth_cache| {
        let duration = now.signed_duration_since(auth_cache.cached_at);
        duration.num_hours() < 24
    });
}

/// JWT认证中间件
pub async fn auth_middleware(
    headers: HeaderMap,
    mut request: Request,
    next: Next,
) -> Result<Response, (StatusCode, Json<ApiResponse<()>>)> {
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
                Json(ApiResponse::error(401, "Missing authorization token".to_string())),
            ));
        }
    };

    // 验证token
    let claims = match verify_token(token) {
        Ok(claims) => claims,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(401, "Invalid token".to_string())),
            ));
        }
    };

    // 将用户ID添加到请求扩展中
    request.extensions_mut().insert(claims.sub);

    Ok(next.run(request).await)
}