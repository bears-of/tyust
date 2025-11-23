use std::{collections::HashMap, time::Duration};

use anyhow::Result;
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use url::Url;

/// 创建不跟随重定向的HTTP客户端
pub fn new_client_no_redirect() -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(Into::into)
}

/// 创建跟随重定向的HTTP客户端
pub fn new_client_follow() -> Result<Client> {
    Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
        .map_err(Into::into)
}

/// 从HeaderMap中获取指定名称的header值
pub fn header_str(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)?
        .to_str()
        .ok()
        .map(ToString::to_string)
}

/// 从Set-Cookie header中提取指定名称的cookie值
/// 
/// # Arguments
/// * `set_cookie_header` - Set-Cookie header的值，可能包含多个cookie
/// * `name` - 要查找的cookie名称
/// 
/// # Returns
/// 返回找到的cookie值，如果未找到则返回None
pub fn get_cookie_value(set_cookie_header: &str, name: &str) -> Option<String> {
    set_cookie_header
        .split(',')
        .find_map(|segment| {
            let segment = segment.trim();
            // 获取cookie的键值对部分（忽略属性）
            let cookie_pair = segment.split(';').next()?;
            let (key, value) = cookie_pair.split_once('=')?;
            
            if key.trim() == name {
                Some(value.trim().to_string())
            } else {
                None
            }
        })
}

/// 构建Cookie header值
/// 
/// # Arguments
/// * `cookies` - cookie键值对的HashMap
/// 
/// # Returns
/// 返回构建好的HeaderValue，如果构建失败则返回错误
pub fn build_cookie_header(cookies: &HashMap<String, String>) -> Result<HeaderValue> {
    if cookies.is_empty() {
        return Ok(HeaderValue::from_static(""));
    }
    
    let cookie_string = cookies
        .iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<_>>()
        .join("; ");
    
    HeaderValue::from_str(&cookie_string).map_err(Into::into)
}

/// 从URL中提取指定的查询参数值
/// 
/// # Arguments
/// * `url` - 要解析的URL字符串
/// * `key` - 要查找的查询参数名称
/// 
/// # Returns
/// 返回找到的参数值，如果URL无效或参数不存在则返回None
#[allow(unused)]
pub fn extract_query_param(url: &str, key: &str) -> Option<String> {
    let parsed_url = Url::parse(url).ok()?;
    parsed_url
        .query_pairs()
        .find(|(k, _)| k == key)
        .map(|(_, v)| v.into_owned())
}