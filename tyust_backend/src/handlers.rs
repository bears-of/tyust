use axum::{
    Json,
    extract::{Extension, Query},
    http::StatusCode,
};
use chrono::NaiveDate;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

use crate::tyust_api::tyust_get_user_info;
use crate::{
    api_types::{
        ApiResponse, Course, LoginParams, ScheduleParams, Score, SemesterConfig,
        SetSemesterStartRequest, UserInfo,
    },
    auth::{UserAuthCache, generate_token},
    de_crypto::get_crypto_and_password,
    entity::UserLoginInfo,
    tyust_api::{
        tyust_get_access_token, tyust_get_current_course, tyust_get_jwglxt_jsession,
        tyust_get_login_code, tyust_get_raw_scores, tyust_get_ronghemenhu_jsessionid,
        tyust_get_route, tyust_get_scores, tyust_get_session,
    },
};

// 全局学期配置存储
lazy_static! {
    static ref SEMESTER_CONFIG: Arc<Mutex<Option<SemesterConfig>>> = Arc::new(Mutex::new(None));
}

/// 用户登录接口
pub async fn login(
    Json(params): Json<LoginParams>,
) -> Result<Json<ApiResponse<UserInfo>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 验证登录信息
    let login_result = authenticate_user(&params.student_id, &params.password).await;

    match login_result {
        Ok(user_name) => {
            // 生成JWT token
            let token = match generate_token(&params.student_id) {
                Ok(token) => token,
                Err(_) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(
                            500,
                            "Failed to generate token".to_string(),
                        )),
                    ));
                }
            };

            // 先尝试从数据库获取现有用户信息（包括头像）
            let db_pool = crate::db::get_db_pool().await;
            let existing_user = crate::db::get_user(db_pool, &params.student_id)
                .await
                .unwrap_or(None);

            let avatar_url = if let Some(user) = &existing_user {
                user.avatar_url.clone()
            } else {
                None
            };

            let user_info = UserLoginInfo {
                student_id: params.student_id.clone(),
                name: user_name,
                class: "未知班级".to_string(), // 可以从API获取更详细信息
                token,
                avatar_url, // 使用数据库中的头像URL（如果存在）
            };

            // 存储用户会话（数据库）
            let db_pool = crate::db::get_db_pool().await;
            if let Err(e) = crate::db::save_user(db_pool, &user_info).await {
                eprintln!("Failed to save user to database: {}", e);
            }

            Ok(Json(ApiResponse::success(user_info)))
        }
        Err(err) => Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(401, format!("Login failed: {}", err))),
        )),
    }
}

/// 获取课表接口
pub async fn get_schedule(
    Extension(student_id): Extension<String>,
    Query(_params): Query<ScheduleParams>,
) -> Result<Json<ApiResponse<Vec<Course>>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 从数据库中获取用户信息
    let db_pool = crate::db::get_db_pool().await;
    let _user_info = match crate::db::get_user(db_pool, &student_id).await {
        Ok(Some(info)) => info,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(
                    401,
                    "User session not found".to_string(),
                )),
            ));
        }
    };

    // 获取课表数据
    match get_user_courses(&student_id).await {
        Ok(courses) => {
            // 前端会根据weeks数组自行过滤，后端直接返回所有课程
            Ok(Json(ApiResponse::success(courses)))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                500,
                format!("Failed to get courses: {}", err),
            )),
        )),
    }
}

/// 获取用户信息接口
pub async fn get_user_info(
    Extension(student_id): Extension<String>,
) -> Result<Json<ApiResponse<UserInfo>>, (StatusCode, Json<ApiResponse<()>>)> {
    let db_pool = crate::db::get_db_pool().await;
    match crate::db::get_user(db_pool, &student_id).await {
        Ok(Some(user_info)) => Ok(Json(ApiResponse::success(user_info))),
        _ => Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                401,
                "User session not found".to_string(),
            )),
        )),
    }
}

/// 用户登出接口
pub async fn logout(Extension(student_id): Extension<String>) -> Json<ApiResponse<()>> {
    // 从数据库中删除
    let db_pool = crate::db::get_db_pool().await;
    let _ = crate::db::delete_user(db_pool, &student_id).await;
    let _ = crate::db::delete_auth_cache(db_pool, &student_id).await;

    Json(ApiResponse::success(()))
}

#[allow(unused)]
/// 设置开学时间接口
pub async fn set_semester_start(
    Json(params): Json<SetSemesterStartRequest>,
) -> Result<Json<ApiResponse<()>>, (StatusCode, Json<ApiResponse<()>>)> {
    let config = SemesterConfig {
        semester_start_date: params.start_date,
        semester_name: params.semester_name,
    };

    // 保存到数据库
    let db_pool = crate::db::get_db_pool().await;
    if let Err(e) = crate::db::save_semester_config(db_pool, &config).await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                500,
                format!("Failed to save semester config to database: {}", e),
            )),
        ));
    }

    // 保存到内存
    match SEMESTER_CONFIG.lock() {
        Ok(mut semester_config) => {
            *semester_config = Some(config);
            Ok(Json(ApiResponse::success(())))
        }
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                500,
                "Failed to set semester config".to_string(),
            )),
        )),
    }
}

#[allow(unused)]
/// 获取开学时间接口
pub async fn get_semester_start()
-> Result<Json<ApiResponse<SemesterConfig>>, (StatusCode, Json<ApiResponse<()>>)> {
    match SEMESTER_CONFIG.lock() {
        Ok(semester_config) => match semester_config.as_ref() {
            Some(config) => Ok(Json(ApiResponse::success(config.clone()))),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    404,
                    "Semester start date not set".to_string(),
                )),
            )),
        },
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                500,
                "Failed to get semester config".to_string(),
            )),
        )),
    }
}

/// 从数据库初始化学期配置
pub async fn init_semester_config() -> Result<(), String> {
    let db_pool = crate::db::get_db_pool().await;

    // 从数据库获取当前激活的学期配置
    let config = crate::db::get_active_semester_config(db_pool)
        .await
        .map_err(|e| format!("Failed to fetch semester config from database: {}", e))?;

    match config {
        Some(config) => match SEMESTER_CONFIG.lock() {
            Ok(mut semester_config) => {
                *semester_config = Some(config);
                Ok(())
            }
            Err(_) => Err("Failed to initialize semester config".to_string()),
        },
        None => {
            Err("No semester configuration found in database. Please set it via API.".to_string())
        }
    }
}

/// 计算当前周次
fn calculate_current_week() -> Option<i32> {
    let semester_config = SEMESTER_CONFIG.lock().ok()?;
    let config = semester_config.as_ref()?;

    let start_date = NaiveDate::parse_from_str(&config.semester_start_date, "%Y-%m-%d").ok()?;
    let current_date = chrono::Local::now().date_naive();

    // 计算两个日期之间的天数差
    let days_diff = (current_date - start_date).num_days();

    // 如果还未开学，返回第1周
    if days_diff < 0 {
        return Some(1);
    }

    // 计算周数（向上取整）
    let week = (days_diff / 7) + 1;

    // 确保周数在合理范围内（1-20周）
    if week >= 1 && week <= 20 {
        Some(week as i32)
    } else if week > 20 {
        Some(20)
    } else {
        Some(1)
    }
}

/// 获取学期配置
pub async fn get_semester_config()
-> Result<Json<ApiResponse<crate::api_types::SemesterConfig>>, (StatusCode, Json<ApiResponse<()>>)>
{
    match SEMESTER_CONFIG.lock() {
        Ok(semester_config) => match semester_config.as_ref() {
            Some(config) => Ok(Json(ApiResponse::success(config.clone()))),
            None => Err((
                StatusCode::NOT_FOUND,
                Json(ApiResponse::error(
                    404,
                    "Semester configuration not found".to_string(),
                )),
            )),
        },
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                500,
                "Failed to get semester config".to_string(),
            )),
        )),
    }
}

/// 认证用户（使用现有的登录逻辑）
async fn authenticate_user(student_id: &str, password: &str) -> Result<String, String> {
    // 使用现有的登录流程进行认证
    let (crypto, password_str) =
        get_crypto_and_password(password).map_err(|e| format!("Crypto error: {}", e))?;

    let (session, execution_code) = tyust_get_session()
        .await
        .map_err(|e| format!("Session error: {}", e))?;

    let (code, _ticket, sourceid_tgc, rg_objectid) = tyust_get_login_code(
        student_id,
        &session,
        &execution_code,
        &crypto,
        &password_str,
    )
    .await
    .map_err(|e| format!("Login error: {}", e))?;

    // 并行执行可以并行的API调用
    let (access_token_result, ronghemenhu_jsession_result) = tokio::join!(
        tyust_get_access_token(&session, &sourceid_tgc, &rg_objectid),
        tyust_get_ronghemenhu_jsessionid(&code)
    );

    let access_token = access_token_result.map_err(|e| format!("Access token error: {}", e))?;

    let ronghemenhu_jsession =
        ronghemenhu_jsession_result.map_err(|e| format!("Ronghemenhu JSESSION error: {}", e))?;

    // 继续执行依赖于access_token的API调用
    let route = tyust_get_route(&access_token)
        .await
        .map_err(|e| format!("Route error: {}", e))?;

    let jwglxt_jsession =
        tyust_get_jwglxt_jsession(&session, &sourceid_tgc, &rg_objectid, &access_token, &route)
            .await
            .map_err(|e| format!("JSESSION error: {}", e))?;

    // 创建用户认证缓存
    let auth_cache = UserAuthCache {
        sourceid_tgc: sourceid_tgc.clone(),
        rg_objectid: rg_objectid.clone(),
        access_token: access_token.clone(),
        route: route.clone(),
        jwglxt_jsession: jwglxt_jsession.clone(),
        ronghemenhu_jsession: ronghemenhu_jsession.clone(),
        code: code.clone(),
        cached_at: chrono::Utc::now(),
    };

    // 只执行一次数据库保存操作
    let db_pool = crate::db::get_db_pool().await;
    if let Err(e) = crate::db::save_auth_cache(db_pool, student_id, &auth_cache).await {
        eprintln!("Failed to save auth cache to database: {}", e);
    }

    let user_info_response = tyust_get_user_info(&ronghemenhu_jsession)
        .await
        .map_err(|e| format!("GET USER_INFO error: {}", e))?;
    Ok(user_info_response.data.name)
}

/// 获取用户课程（使用缓存的认证信息）
async fn get_user_courses(student_id: &str) -> Result<Vec<Course>, String> {
    // 从数据库中获取认证信息
    let db_pool = crate::db::get_db_pool().await;
    let auth_cache = crate::db::get_auth_cache(db_pool, student_id)
        .await
        .ok()
        .flatten()
        .ok_or_else(|| "User auth cache not found. Please login again.".to_string())?;

    // 检查缓存是否有效（24小时内）
    use crate::auth::is_auth_cache_valid;
    if !is_auth_cache_valid(&auth_cache) {
        return Err("Auth cache expired. Please login again.".to_string());
    }

    // 使用缓存的认证信息获取课程数据
    let kb_list = tyust_get_current_course(
        &auth_cache.jwglxt_jsession,
        &auth_cache.access_token,
        &auth_cache.route,
    )
    .await
    .map_err(|e| format!("Course error: {}", e))?;

    // 转换为Course格式
    let courses: Vec<Course> = kb_list.into_iter().map(Course::from).collect();

    Ok(courses)
}

/// 获取课程列表接口（不过滤周次）
pub async fn get_courses(
    headers: axum::http::HeaderMap,
) -> Result<Json<ApiResponse<Vec<Course>>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 从 header 中获取 token
    let token = match headers.get("token") {
        Some(header_value) => match header_value.to_str() {
            Ok(t) => t,
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::error(
                        400,
                        "Invalid token header value".to_string(),
                    )),
                ));
            }
        },
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(400, "Missing token header".to_string())),
            ));
        }
    };

    // 从 header 中获取 student_id
    let student_id = match headers.get("studentId") {
        Some(header_value) => match header_value.to_str() {
            Ok(id) => id,
            Err(_) => {
                return Err((
                    StatusCode::BAD_REQUEST,
                    Json(ApiResponse::error(
                        400,
                        "Invalid studentId header value".to_string(),
                    )),
                ));
            }
        },
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(
                    400,
                    "Missing studentId header".to_string(),
                )),
            ));
        }
    };

    // 验证token
    use crate::auth::verify_token;
    let claims = match verify_token(token) {
        Ok(claims) => claims,
        Err(_) => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(401, "Invalid token".to_string())),
            ));
        }
    };

    // 确保token中的student_id与请求的student_id匹配
    if claims.sub != student_id {
        return Err((
            StatusCode::FORBIDDEN,
            Json(ApiResponse::error(
                403,
                "Token does not match student_id".to_string(),
            )),
        ));
    }

    // 从数据库中获取用户信息
    let db_pool = crate::db::get_db_pool().await;
    let _user_info = match crate::db::get_user(db_pool, student_id).await {
        Ok(Some(info)) => info,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(
                    401,
                    "User session not found".to_string(),
                )),
            ));
        }
    };

    // 获取课程数据
    match get_user_courses(student_id).await {
        Ok(courses) => Ok(Json(ApiResponse::success(courses))),
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                500,
                format!("Failed to get courses: {}", err),
            )),
        )),
    }
}

/// 获取有效成绩接口
pub async fn get_scores(
    Extension(student_id): Extension<String>,
) -> Result<Json<ApiResponse<Vec<Score>>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 从数据库中获取用户信息
    let db_pool = crate::db::get_db_pool().await;
    let _user_info = match crate::db::get_user(db_pool, &student_id).await {
        Ok(Some(info)) => info,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(
                    401,
                    "User session not found".to_string(),
                )),
            ));
        }
    };

    // 从数据库中获取认证信息
    let auth_cache = match crate::db::get_auth_cache(db_pool, &student_id).await {
        Ok(Some(cache)) => cache,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(
                    401,
                    "User auth cache not found. Please login again.".to_string(),
                )),
            ));
        }
    };

    // 检查缓存是否有效
    use crate::auth::is_auth_cache_valid;
    if !is_auth_cache_valid(&auth_cache) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                401,
                "Auth cache expired. Please login again.".to_string(),
            )),
        ));
    }

    // 使用缓存的认证信息获取成绩数据
    match tyust_get_scores(
        &auth_cache.jwglxt_jsession,
        &auth_cache.access_token,
        &auth_cache.route,
    )
    .await
    {
        Ok(score_items) => {
            let scores: Vec<Score> = score_items.into_iter().map(Score::from).collect();
            Ok(Json(ApiResponse::success(scores)))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                500,
                format!("Failed to get scores: {}", err),
            )),
        )),
    }
}

/// 获取原始成绩接口
pub async fn get_raw_scores(
    Extension(student_id): Extension<String>,
    Query(params): Query<crate::api_types::RawScoresParams>,
) -> Result<Json<ApiResponse<Vec<Score>>>, (StatusCode, Json<ApiResponse<()>>)> {
    // 从数据库中获取用户信息
    let db_pool = crate::db::get_db_pool().await;
    let _user_info = match crate::db::get_user(db_pool, &student_id).await {
        Ok(Some(info)) => info,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(
                    401,
                    "User session not found".to_string(),
                )),
            ));
        }
    };

    // 从数据库中获取认证信息
    let auth_cache = match crate::db::get_auth_cache(db_pool, &student_id).await {
        Ok(Some(cache)) => cache,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(
                    401,
                    "User auth cache not found. Please login again.".to_string(),
                )),
            ));
        }
    };

    // 检查缓存是否有效
    use crate::auth::is_auth_cache_valid;
    if !is_auth_cache_valid(&auth_cache) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ApiResponse::error(
                401,
                "Auth cache expired. Please login again.".to_string(),
            )),
        ));
    }

    // 使用缓存的认证信息获取原始成绩数据
    // 使用查询参数或默认为学号和空字符串
    let xh_id = params.xh_id.as_deref().unwrap_or(&student_id);
    let xnm = params.xnm.as_deref().unwrap_or("");
    let xqm = params.xqm.as_deref().unwrap_or("");

    match tyust_get_raw_scores(
        &auth_cache.jwglxt_jsession,
        &auth_cache.route,
        xh_id,
        xnm,
        xqm,
    )
    .await
    {
        Ok(score_items) => {
            let scores: Vec<Score> = score_items.into_iter().map(Score::from).collect();
            Ok(Json(ApiResponse::success(scores)))
        }
        Err(err) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ApiResponse::error(
                500,
                format!("Failed to get raw scores: {}", err),
            )),
        )),
    }
}

/// 初始化登录(获取验证码相关信息)
pub async fn init_login()
-> Result<Json<ApiResponse<crate::api_types::LoginInitData>>, (StatusCode, Json<ApiResponse<()>>)> {
    use crate::api_types::LoginInitData;

    // 简化实现: 生成一个模拟的cookie和formData
    // 前端会使用这些数据,但实际登录时后端不会真正验证验证码
    let cookie = format!("SESSION_{}", chrono::Utc::now().timestamp_millis());
    let form_data = serde_json::json!({
        "timestamp": chrono::Utc::now().timestamp_millis(),
        "initialized": true
    });

    let data = LoginInitData { cookie, form_data };

    Ok(Json(ApiResponse::success(data)))
}

/// 获取验证码图片
pub async fn get_login_code(
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> Vec<u8> {
    // 简化实现: 返回一个简单的1x1像素PNG图片
    // 前端会显示这个"验证码",但实际不需要用户输入
    let _cookie = params.get("cookie");

    // 1x1 transparent PNG
    vec![
        0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, // PNG signature
        0x00, 0x00, 0x00, 0x0D, // IHDR length
        0x49, 0x48, 0x44, 0x52, // IHDR
        0x00, 0x00, 0x00, 0x01, // width = 1
        0x00, 0x00, 0x00, 0x01, // height = 1
        0x08, 0x06, 0x00, 0x00, 0x00, // bit depth, color type, etc.
        0x1F, 0x15, 0xC4, 0x89, // CRC
        0x00, 0x00, 0x00, 0x0A, // IDAT length
        0x49, 0x44, 0x41, 0x54, // IDAT
        0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00, 0x05, 0x00, 0x01, // data
        0x0D, 0x0A, 0x2D, 0xB4, // CRC
        0x00, 0x00, 0x00, 0x00, // IEND length
        0x49, 0x45, 0x4E, 0x44, // IEND
        0xAE, 0x42, 0x60, 0x82, // CRC
    ]
}

/// 更新头像请求参数
#[derive(serde::Deserialize)]
pub struct UpdateAvatarParams {
    #[serde(rename = "avatarData")]
    pub avatar_data: String,
}

/// 更新用户头像
pub async fn update_avatar(
    Extension(student_id): Extension<String>,
    Json(params): Json<UpdateAvatarParams>,
) -> Result<Json<ApiResponse<serde_json::Value>>, (StatusCode, Json<ApiResponse<()>>)> {
    use base64::{Engine as _, engine::general_purpose};
    use std::fs::File;
    use std::io::Write;

    let db_pool = crate::db::get_db_pool().await;

    // 获取当前用户信息
    let mut user_info = match crate::db::get_user(db_pool, &student_id).await {
        Ok(Some(user)) => user,
        _ => {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ApiResponse::error(401, "User not found".to_string())),
            ));
        }
    };

    // 解码base64数据
    let base64_data = params.avatar_data.replace("data:image/jpeg;base64,", "");
    let base64_data = base64_data.replace("data:image/png;base64,", "");

    match general_purpose::STANDARD.decode(&base64_data) {
        Ok(image_data) => {
            // 生成唯一的文件名
            let filename = format!("avatar_{}.jpg", chrono::Utc::now().timestamp());
            let file_path = format!("static/avatars/{}", filename);

            // 确保目录存在
            std::fs::create_dir_all("static/avatars").ok();

            // 保存文件
            match File::create(&file_path) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(&image_data) {
                        eprintln!("Failed to write avatar file: {}", e);
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::error(
                                500,
                                "Failed to save avatar file".to_string(),
                            )),
                        ));
                    }

                    // 构建可访问的URL
                    let avatar_url = format!("/static/avatars/{}", filename);

                    // 更新头像URL
                    user_info.set_avatar_url(Some(avatar_url.clone()));

                    // 保存到数据库
                    if let Err(e) = crate::db::save_user(db_pool, &user_info).await {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiResponse::error(
                                500,
                                format!("Failed to update avatar: {}", e),
                            )),
                        ));
                    }

                    // 返回新的头像URL
                    let response_data = serde_json::json!({
                        "avatarUrl": avatar_url
                    });

                    Ok(Json(ApiResponse::success(response_data)))
                }
                Err(e) => {
                    eprintln!("Failed to create avatar file: {}", e);
                    Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiResponse::error(
                            500,
                            "Failed to create avatar file".to_string(),
                        )),
                    ))
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to decode base64 avatar data: {}", e);
            Err((
                StatusCode::BAD_REQUEST,
                Json(ApiResponse::error(400, "Invalid avatar data".to_string())),
            ))
        }
    }
}
