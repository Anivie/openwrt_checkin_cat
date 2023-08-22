use reqwest::header::{HeaderMap, HeaderValue};

pub fn token_header(content_length: &usize) -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_static("dukou.to"));
    headers.insert("method", HeaderValue::from_static("POST"));
    headers.insert("path", HeaderValue::from_static("/api/token"));
    headers.insert("scheme", HeaderValue::from_static("https"));
    headers.insert("Accept", HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    headers.insert("Accept-Language", HeaderValue::from_static("zh-CN,zh;q=0.9"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert("Content-Length", content_length.to_string().parse().unwrap());
    headers.insert("Content-Type", HeaderValue::from_static("application/json;charset=UTF-8"));
    headers.insert("Origin", HeaderValue::from_static("https://dukou.to"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    headers.insert("Referer", HeaderValue::from_static("https://dukou.to/user/login?redirect=%2Fuser%2Findex"));
    headers.insert("Sec-Ch-Ua", HeaderValue::from_static("\"Not/A)Brand\";v=\"99\", \"Google Chrome\";v=\"115\", \"Chromium\";v=\"115\""));
    headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36"));
    headers
}

pub fn checkin_header(token: &str) -> HeaderMap {
    let mut headers: HeaderMap = HeaderMap::new();
    headers.insert("authority", HeaderValue::from_static("dukou.to"));
    headers.insert("method", HeaderValue::from_static("POST"));
    headers.insert("path", HeaderValue::from_static("/api/token"));
    headers.insert("scheme", HeaderValue::from_static("https"));
    headers.insert("Accept", HeaderValue::from_static("application/json, text/plain, */*"));
    headers.insert("Access-Token", token.parse().unwrap());
    headers.insert("Accept-Encoding", HeaderValue::from_static("gzip, deflate, br"));
    headers.insert("Accept-Language", HeaderValue::from_static("zh-CN,zh;q=0.9"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    // headers.insert("Content-Length", HeaderValue::from_static("53"));
    headers.insert("Content-Type", HeaderValue::from_static("application/json;charset=UTF-8"));
    headers.insert("Origin", HeaderValue::from_static("https://dukou.to"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    headers.insert("Referer", HeaderValue::from_static("https://dukou.to/user/login?redirect=%2Fuser%2Findex"));
    headers.insert("Sec-Ch-Ua", HeaderValue::from_static("\"Not/A)Brand\";v=\"99\", \"Google Chrome\";v=\"115\", \"Chromium\";v=\"115\""));
    headers.insert("Sec-Ch-Ua-Mobile", HeaderValue::from_static("?0"));
    headers.insert("Sec-Ch-Ua-Platform", HeaderValue::from_static("\"Windows\""));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-origin"));
    headers.insert("User-Agent", HeaderValue::from_static("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/115.0.0.0 Safari/537.36"));
    headers
}