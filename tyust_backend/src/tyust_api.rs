use crate::de_crypto;
use crate::entity::{self, RonghemenhuUserInfoResponse, TyustCourseResponse, TyustScoreResponse};
use crate::http_helper::{
    build_cookie_header, extract_query_param, get_cookie_value, header_str, new_client_follow,
    new_client_no_redirect,
};
use anyhow::{Context, Ok, Result, anyhow};
use once_cell::sync::Lazy;
use rand::RngCore;
use regex::Regex;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue, LOCATION, REFERER, SET_COOKIE, USER_AGENT};
use serde_json::json;
use std::collections::HashMap;
use url::Url;

// 全局HTTP客户端
static CLIENT_FOLLOW: Lazy<reqwest::Client> =
    Lazy::new(|| new_client_follow().expect("Failed to create follow client"));
static CLIENT_NO_REDIRECT: Lazy<reqwest::Client> =
    Lazy::new(|| new_client_no_redirect().expect("Failed to create no-redirect client"));

pub async fn tyust_get_session() -> Result<(String, String)> {
    let resp = CLIENT_FOLLOW
        .get("https://sso1.tyust.edu.cn/login")
        .send()
        .await
        .context("GET sso login")?;
    let set_cookie = header_str(resp.headers(), SET_COOKIE.as_str())
        .ok_or_else(|| anyhow!("SESSION cookie missing"))?;
    let session = get_cookie_value(&set_cookie, "SESSION")
        .ok_or_else(|| anyhow!("SESSION not found in Set-Cookie"))?;
    let text = resp.text().await.context("read login page")?;
    let re = Regex::new(r#"<p id="login-page-flowkey">(.*?)</p>"#).unwrap();
    let execution_code = re
        .captures(&text)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .ok_or_else(|| anyhow!("execution_code not found"))?;
    Ok((session, execution_code))
}

pub async fn tyust_get_ronghemenhu_jsessionid(code: &str) -> Result<String> {
    let url = "https://ronghemenhu.tyust.edu.cn/portal/publish/web/login/loginByOauth";
    let payload = json!({
        "code": code,
        "username": "",
        "password": ""
    });
    let resp = CLIENT_NO_REDIRECT.post(url).json(&payload).send().await?;
    let set_cookie = header_str(resp.headers(), SET_COOKIE.as_str())
        .ok_or_else(|| anyhow!("Set-Cookie missing for JSESSIONID"))?;
    let jsessionid = get_cookie_value(&set_cookie, "JSESSIONID")
        .ok_or_else(|| anyhow!("JSESSIONID not found"))?;

    Ok(jsessionid)
}

pub async fn tyust_get_login_code(
    username: &str,
    session: &str,
    execution_code: &str,
    crypto: &str,
    password_str: &str,
) -> Result<(String, String, String, String)> {
    let login_url = "https://sso1.tyust.edu.cn/login";

    let mut headers = HeaderMap::new();
    let mut cookies = HashMap::new();
    cookies.insert("SESSION".into(), session.into());
    headers.insert(COOKIE, build_cookie_header(&cookies)?);
    let form = vec![
        ("username", username.to_string()),
        ("type", "UsernamePassword".into()),
        ("_eventId", "submit".into()),
        ("geolocation", "".into()),
        ("execution", execution_code.to_string()),
        ("captcha_code", "".into()),
        ("croypto", crypto.to_string()),
        ("password", password_str.to_string()),
    ];
    let resp_no = CLIENT_NO_REDIRECT
        .post(login_url)
        .headers(headers.clone())
        .form(&form)
        .send()
        .await
        .context("post login (no redirect)")?;
    let (ticket, sourceid_tgc, rg_objectid) = handle_login_information(resp_no.headers())?;
    let next_location = resp_no.headers().get("Location").unwrap().to_str()?;
    let res = CLIENT_NO_REDIRECT
        .get(next_location)
        .send()
        .await
        .context("get next redirect")?;
    let next_location = res.headers().get("Location").unwrap().to_str()?;

    cookies.insert("SOURCEID_TGC".into(), sourceid_tgc.clone());
    cookies.insert("rg_objectid".into(), rg_objectid.clone());
    headers.insert(COOKIE, build_cookie_header(&cookies)?);
    let resp_follow = CLIENT_FOLLOW
        .get(next_location)
        .headers(headers)
        .send()
        .await
        .context("post login (follow)")?;

    let final_url = resp_follow.url().to_string();
    let code = extract_query_param(&final_url, "code")
        .ok_or_else(|| anyhow!("code not found in final url"))?;
    Ok((code, ticket, sourceid_tgc, rg_objectid))
}

#[allow(unused)]
// get_ronghemenhu_jsessionid
pub async fn tyust_get_user_info(jsessionid: &str) -> Result<RonghemenhuUserInfoResponse> {
    let mut headers = HeaderMap::new();
    let mut cookies = HashMap::new();
    cookies.insert("JSESSIONID".into(), jsessionid.into());
    headers.insert(COOKIE, build_cookie_header(&cookies)?);

    let resp = CLIENT_FOLLOW
        .get("https://ronghemenhu.tyust.edu.cn/portal/publish/web/login/user")
        .headers(headers)
        .send()
        .await?;
    Ok(resp.json::<RonghemenhuUserInfoResponse>().await?)
}

pub fn generate_device_id() -> String {
    let mut b = [0u8; 16];
    rand::rng().fill_bytes(&mut b);
    b.iter().map(|x| format!("{:02x}", x)).collect()
}

pub async fn tyust_get_access_token(
    session: &str,
    sourceid_tgc: &str,
    rg_objectid: &str,
) -> Result<String> {
    let mut headers = HeaderMap::new();
    let mut cookies = HashMap::new();
    cookies.insert("SESSION".into(), session.into());
    cookies.insert("SOURCEID_TGC".into(), sourceid_tgc.into());
    cookies.insert("rg_objectid".into(), rg_objectid.into());
    headers.insert(COOKIE, build_cookie_header(&cookies)?);

    let resp = CLIENT_NO_REDIRECT
        .get("https://sso1.tyust.edu.cn/login?service=https://zero.tyust.edu.cn/login/casCallback/r3IveGXj/")
        .headers(headers)
        .send()
        .await?;
    let loc = header_str(resp.headers(), LOCATION.as_str())
        .ok_or_else(|| anyhow!("Location missing for ticket"))?;
    let ticket =
        extract_query_param(&loc, "ticket").ok_or_else(|| anyhow!("ticket missing in redirect"))?;

    let payload = json!({
        "externalId": "r3IveGXj",
        "data": serde_json::to_string(&json!({
            "callbackUrl": "https://zero.tyust.edu.cn/login/casCallback/r3IveGXj/",
            "ticket": ticket,
            "deviceId": generate_device_id(),
        }))?
    });
    let resp2 = CLIENT_FOLLOW
        .post("https://zero.tyust.edu.cn/api/access/auth/finish")
        .json(&payload)
        .send()
        .await?;
    let v = resp2.json::<serde_json::Value>().await?;
    let token = v["data"]["token"]
        .as_str()
        .ok_or_else(|| anyhow!("token missing"))?
        .to_string();
    Ok(token)
}

pub async fn tyust_get_route(access_token: &str) -> Result<String> {
    let mut headers = HeaderMap::new();
    let mut cookies = HashMap::new();
    cookies.insert("__access_token".into(), access_token.into());
    headers.insert(COOKIE, build_cookie_header(&cookies)?);

    let resp = CLIENT_NO_REDIRECT
        .get("https://newjwc.tyust.edu.cn/sso/jasiglogin/jwglxt")
        .headers(headers)
        .send()
        .await?;
    let set_cookie = header_str(resp.headers(), SET_COOKIE.as_str())
        .ok_or_else(|| anyhow!("route Set-Cookie missing"))?;
    let route =
        get_cookie_value(&set_cookie, "route").ok_or_else(|| anyhow!("route cookie not found"))?;
    Ok(route)
}

fn handle_login_information(headers: &HeaderMap) -> Result<(String, String, String)> {
    let location = headers
        .get("Location")
        .ok_or_else(|| anyhow!("locations missing"))?
        .to_str()?;
    let ticket =
        extract_query_param(&location, "ticket").ok_or_else(|| anyhow!("ticket missing"))?;

    let set_cookies = headers.get_all(SET_COOKIE);
    let mut rg_objectid: String = String::new();
    let mut sourceid_tgc: String = String::new();
    let re = Regex::new(r"rg_objectid=([a-zA-Z0-9]+)").unwrap();
    for cookies in set_cookies.iter() {
        if let Some(temp_sourceid) = get_cookie_value(&cookies.to_str()?, "SOURCEID_TGC") {
            sourceid_tgc = temp_sourceid;
        }
        if let Some(temp_objectid) = re
            .captures(cookies.to_str()?)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
        {
            rg_objectid = temp_objectid;
        }
    }
    Ok((ticket, sourceid_tgc, rg_objectid))
}

pub async fn follow_redirects_for_jsession(
    mut url: String,
    mut cookies: HashMap<String, String>,
    hops: usize,
) -> Result<String> {
    for _ in 0..hops {
        let mut headers = HeaderMap::new();
        headers.insert(COOKIE, build_cookie_header(&cookies)?);
        let resp = CLIENT_NO_REDIRECT.get(&url).headers(headers).send().await?;

        // 收集新的 cookie 它会先返回一个验证的jsession然后再跳转返回真正需要的jsession
        let mut skip_first = false;
        if let Some(sc) = header_str(resp.headers(), SET_COOKIE.as_str()) {
            for seg in sc.split(',') {
                if let Some(eq) = seg.split(';').next() {
                    let mut kv = eq.trim().splitn(2, '=');
                    if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                        cookies.insert(k.trim().to_string(), v.trim().to_string());
                        if k.trim() == "JSESSIONID" {
                            if skip_first == true {
                                return Ok(v.trim().to_string());
                            }
                            skip_first = true;
                        }
                    }
                }
            }
        }

        if let Some(loc) = header_str(resp.headers(), LOCATION.as_str()) {
            let next = if Url::parse(&loc).is_ok() {
                loc
            } else {
                let base = Url::parse(&url).unwrap();
                base.join(&loc).unwrap().to_string()
            };
            url = next;
        } else {
            break;
        }
    }

    if let Some(js) = cookies.get("JSESSIONID") {
        return Ok(js.clone());
    }
    Err(anyhow!("JSESSIONID not obtained after redirects"))
}

pub async fn tyust_get_jwglxt_jsession(
    session: &str,
    sourceid_tgc: &str,
    rg_objectid: &str,
    access_token: &str,
    route: &str,
) -> Result<String> {
    let mut cookies = HashMap::new();
    cookies.insert("SESSION".into(), session.into());
    cookies.insert("SOURCEID_TGC".into(), sourceid_tgc.into());
    cookies.insert("rg_objectid".into(), rg_objectid.into());
    cookies.insert("__access_token".into(), access_token.into());
    cookies.insert("route".into(), route.into());

    let start =
        "https://sso1.tyust.edu.cn/login?service=https://newjwc.tyust.edu.cn/sso/jasiglogin/jwglxt";
    let jsession = follow_redirects_for_jsession(start.to_string(), cookies, 10).await?;

    Ok(jsession)
}

pub async fn tyust_get_current_course(
    jwglxt_jsession: &str,
    access_token: &str,
    route: &str,
) -> Result<Vec<entity::Kb>> {
    let mut headers = HeaderMap::new();
    let mut cookies = HashMap::new();
    cookies.insert("__access_token".into(), access_token.into());
    cookies.insert("JSESSIONID".into(), jwglxt_jsession.into());
    cookies.insert("route".into(), route.into());
    headers.insert(COOKIE, build_cookie_header(&cookies)?);
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/115.0"),
    );
    headers.insert(REFERER, HeaderValue::from_static("https://newjwc.tyust.edu.cn/jwglxt/kbcx/xskbcx_cxXskbcxIndex.html?gnmkdm=N253508&layout=default"));
    let params = [("gnmkdm", "N253508")];
    let form = [("xnm", "2025"), ("xqm", "3"), ("kzlx", "ck"), ("xsdm", "")];
    let resp = CLIENT_NO_REDIRECT
        .post("https://newjwc.tyust.edu.cn/jwglxt/kbcx/xskbcx_cxXsgrkb.html")
        .headers(headers)
        .query(&params)
        .form(&form)
        .send()
        .await
        .context("post xskbcx")?;

    let kb_list = resp.json::<TyustCourseResponse>().await?.kb_list;
    Ok(kb_list)
}

/// 获取有效成绩
pub async fn tyust_get_scores(
    jwglxt_jsession: &str,
    access_token: &str,
    route: &str,
) -> Result<Vec<entity::ScoreItem>> {
    let mut headers = HeaderMap::new();
    let mut cookies = HashMap::new();
    cookies.insert("__access_token".into(), access_token.into());
    cookies.insert("JSESSIONID".into(), jwglxt_jsession.into());
    cookies.insert("route".into(), route.into());
    headers.insert(COOKIE, build_cookie_header(&cookies)?);
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/115.0"),
    );
    headers.insert(REFERER, HeaderValue::from_static("https://newjwc.tyust.edu.cn/jwglxt/cjcx/cjcx_cxDgXscj.html?gnmkdm=N305005&layout=default"));

    let params = [("gnmkdm", "N305005")];
    let form = [
        ("xnm", ""),
        ("xqm", ""),
        ("_search", "false"),
        ("nd", &chrono::Utc::now().timestamp_millis().to_string()),
        ("queryModel.showCount", "5000"),
        ("queryModel.currentPage", "1"),
        ("queryModel.sortName", ""),
        ("queryModel.sortOrder", "asc"),
        ("time", "1"),
    ];

    let resp = CLIENT_NO_REDIRECT
        .post("https://newjwc.tyust.edu.cn/jwglxt/cjcx/cjcx_cxDgXscj.html")
        .headers(headers)
        .query(&params)
        .form(&form)
        .send()
        .await
        .context("post cjcx")?;

    let score_response = resp.json::<TyustScoreResponse>().await?;
    Ok(score_response.items)
}

/// 获取原始成绩
pub async fn tyust_get_raw_scores(
    jwglxt_jsession: &str,
    access_token: &str,
    route: &str,
) -> Result<Vec<entity::ScoreItem>> {
    let mut headers = HeaderMap::new();
    let mut cookies = HashMap::new();
    cookies.insert("__access_token".into(), access_token.into());
    cookies.insert("JSESSIONID".into(), jwglxt_jsession.into());
    cookies.insert("route".into(), route.into());
    headers.insert(COOKIE, build_cookie_header(&cookies)?);
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) Chrome/115.0"),
    );
    headers.insert(REFERER, HeaderValue::from_static("https://newjwc.tyust.edu.cn/jwglxt/cjcx/cjcx_cxXsYscj.html?gnmkdm=N305007&layout=default"));

    let params = [("gnmkdm", "N305007")];
    let form = [
        ("xnm", ""),
        ("xqm", ""),
        ("_search", "false"),
        ("nd", &chrono::Utc::now().timestamp_millis().to_string()),
        ("queryModel.showCount", "5000"),
        ("queryModel.currentPage", "1"),
        ("queryModel.sortName", ""),
        ("queryModel.sortOrder", "asc"),
        ("time", "1"),
    ];

    let resp = CLIENT_NO_REDIRECT
        .post("https://newjwc.tyust.edu.cn/jwglxt/cjcx/cjcx_cxXsYscj.html")
        .headers(headers)
        .query(&params)
        .form(&form)
        .send()
        .await
        .context("post cjcx raw")?;

    let score_response = resp.json::<TyustScoreResponse>().await?;
    Ok(score_response.items)
}
