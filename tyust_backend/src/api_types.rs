use serde::{Deserialize, Serialize};
use crate::entity::{Kb, ScoreItem};

// 重新导出FullUserInfo作为UserInfo以保持API兼容性
pub use crate::entity::UserLoginInfo as UserInfo;

/// 统一API响应格式
#[derive(Debug, Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            message: "success".to_string(),
            data: Some(data),
        }
    }

    pub fn error(code: i32, message: String) -> Self {
        Self {
            code,
            message,
            data: None,
        }
    }
}

/// 登录请求参数
#[derive(Debug, Deserialize)]
pub struct LoginParams {
    #[serde(rename = "stuId")]
    pub student_id: String,
    pub password: String,
}

/// 验证码登录请求参数
#[derive(Debug, Deserialize)]
pub struct LoginWithVerifyParams {
    #[serde(rename = "stuId")]
    pub student_id: String,
    pub password: String,
    #[serde(rename = "verifyCode")]
    pub verify_code: String,
    pub cookie: String,
    #[serde(rename = "formData")]
    pub form_data: String,
}

/// 登录初始化响应数据
#[derive(Debug, Serialize)]
pub struct LoginInitData {
    pub cookie: String,
    #[serde(rename = "formData")]
    pub form_data: serde_json::Value,
}

/// 验证码登录响应数据
#[derive(Debug, Serialize)]
pub struct LoginWithVerifyData {
    pub cookie: String,
}

/// 课程信息（转换自Kb结构）
#[derive(Debug, Serialize)]
pub struct Course {
    pub id: String,
    pub name: String,
    pub teacher: String,
    pub classroom: String,
    pub time: String,
    pub week: i32, // 星期几 (1-7)
    pub section: i32, // 开始节次
    #[serde(rename = "sectionCount")]
    pub section_count: i32, // 持续节次数
    pub weeks: Vec<i32>, // 周次数组，如 [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]
    // 课程详情页所需字段
    #[serde(rename = "rawWeeks")]
    pub raw_weeks: String, // 原始周次字符串，如 "1-16周"
    #[serde(rename = "rawSection")]
    pub raw_section: String, // 原始节次字符串，如 "一 1-2"
    pub address: String, // 上课地点（教室）
    pub credit: String, // 学分
    pub category: String, // 课程类别
    pub method: String, // 考核方式
}

impl From<Kb> for Course {
    fn from(kb: Kb) -> Self {
        // 解析星期几 (1-7)
        let week = kb.xqj.parse::<i32>().unwrap_or(1);
        
        // 解析节次信息，如 "1-2" -> section=1, section_count=2
        let (section, section_count) = parse_section(&kb.jc);
        
        // 解析周次范围，如 "1-16周" -> [1,2,3,...,16]
        let weeks = parse_weeks_range(&kb.zcd);

        Self {
            id: kb.kch_id,
            name: kb.kcmc,
            teacher: kb.xm,
            classroom: kb.cdmc.clone(),
            time: format!("{} {}", kb.xqjmc, kb.jc),
            week,
            section,
            section_count,
            weeks,
            // 课程详情页字段
            raw_weeks: kb.zcd.clone(),
            raw_section: kb.jc.clone(),
            address: kb.cdmc,
            credit: kb.xf,
            category: kb.kclb,
            method: kb.khfsmc,
        }
    }
}

/// 解析节次字符串，如 "1-2" -> (1, 2)
fn parse_section(jc: &str) -> (i32, i32) {
    let parts: Vec<&str> = jc.split('-').collect();
    if parts.len() == 2 {
        let start = parts[0].parse::<i32>().unwrap_or(1);
        let end = parts[1].parse::<i32>().unwrap_or(start);
        let count = end - start + 1;
        (start, count)
    } else {
        let section = jc.parse::<i32>().unwrap_or(1);
        (section, 1)
    }
}

/// 解析周次范围字符串，如 "1-16周" -> [1,2,3,...,16]
fn parse_weeks_range(zcd: &str) -> Vec<i32> {
    let clean_str = zcd.replace('周', "");
    
    // 处理单周/双周的情况，如 "1-16周(单)"
    let base_str = if clean_str.contains('(') {
        clean_str.split('(').next().unwrap_or(&clean_str)
    } else {
        &clean_str
    };
    
    let is_odd = clean_str.contains("单");
    let is_even = clean_str.contains("双");
    
    // 处理逗号分隔的情况，如 "1,3,5-8周"
    let mut all_weeks = Vec::new();
    
    for part in base_str.split(',') {
        if part.contains('-') {
            let range_parts: Vec<&str> = part.split('-').collect();
            if range_parts.len() == 2 {
                if let (Ok(start), Ok(end)) = (
                    range_parts[0].trim().parse::<i32>(),
                    range_parts[1].trim().parse::<i32>(),
                ) {
                    for week in start..=end {
                        if is_odd && week % 2 == 0 {
                            continue;
                        }
                        if is_even && week % 2 == 1 {
                            continue;
                        }
                        if !all_weeks.contains(&week) {
                            all_weeks.push(week);
                        }
                    }
                }
            }
        } else if let Ok(week) = part.trim().parse::<i32>() {
            if !all_weeks.contains(&week) {
                all_weeks.push(week);
            }
        }
    }
    
    all_weeks.sort();
    all_weeks
}

/// 获取课表请求参数
#[derive(Debug, Deserialize)]
pub struct ScheduleParams {
    pub week: Option<i32>,
}

/// 开学时间配置
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SemesterConfig {
    pub semester_start_date: String, // 开学日期，格式: "2024-02-26"
    pub semester_name: String, // 学期名称，如"2023-2024学年第二学期"
}

/// 设置开学时间的请求参数
#[derive(Debug, Serialize, Deserialize)]
pub struct SetSemesterStartRequest {
    pub start_date: String, // 开学日期，格式: "2024-02-26"
    pub semester_name: String, // 学期名称
}

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String, // 学号
    pub exp: usize,  // 过期时间
    pub iat: usize,  // 签发时间
}

/// 管理员 JWT Claims
#[derive(Debug, Serialize, Deserialize)]
pub struct AdminClaims {
    pub sub: String, // 管理员ID
    pub username: String, // 管理员用户名
    pub exp: usize,  // 过期时间
    pub iat: usize,  // 签发时间
}

/// 成绩信息（API响应格式）
#[derive(Debug, Serialize)]
pub struct Score {
    pub semester: String, // 学期，如"2024-2025学年第一学期"
    pub course: String,   // 课程名称
    pub credit: String,   // 学分
    pub score: String,    // 成绩
    pub gpa: String,      // 绩点
    pub teacher: String,  // 教师
    #[serde(rename = "courseType")]
    pub course_type: String, // 课程类型
}

impl From<ScoreItem> for Score {
    fn from(item: ScoreItem) -> Self {
        Self {
            semester: format!("{} {}", item.xnmmc, item.xqmmc),
            course: item.kcmc,
            credit: item.xf,
            score: item.cj,
            gpa: item.jd,
            teacher: item.jsxm,
            course_type: item.kcxzmc,
        }
    }
}