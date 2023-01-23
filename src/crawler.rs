use std::sync::Arc;
use std::time::Duration;

use reqwest::header::HeaderValue;
use reqwest::Method;
use tokio::sync::mpsc::Sender;
use tokio::task::JoinHandle;

const JS_REGEX: &str = "(https://shared.ydstatic.com/.*.js)";
const TOKEN_REGEX: &str = r#"n.md5\("fanyideskweb".+?"(.+?)"\)"#;

pub struct Crawler {
    handle: Option<JoinHandle<()>>,
}

impl Crawler {
    pub async fn new(user_agent: String, caching_time: Duration, sender: Arc<Sender<String>>) {
        let mut interval = tokio::time::interval(caching_time);
        let client = reqwest::Client::new();
        loop {
            let token = Self::run(&client, &user_agent.clone()).await;
            if token.is_some() {
                let res = sender.send(token.unwrap()).await;
                if res.is_err() {
                    break;
                }
            }
            interval.tick().await;
        }
    }

    async fn run(client: &reqwest::Client, ua: &str) -> Option<String> {
        let js = Self::get_js(client, ua).await;
        match js {
            None => None,
            Some(url) => Self::get_token(client, ua, &url).await,
        }
    }

    async fn get_js(client: &reqwest::Client, ua: &str) -> Option<String> {
        let request = Self::new_request(ua, "https://fanyi.youdao.com/");
        let resp = client.execute(request.try_clone().unwrap()).await.unwrap();
        if resp.status().as_u16() != 200 {
            return None;
        }
        let regex = regex::Regex::new(JS_REGEX).unwrap();
        let body = resp.text().await.unwrap();
        if !regex.is_match(body.as_str()) {
            return None;
        }
        let match_res = regex.find(body.as_str()).unwrap();
        Some(match_res.as_str().to_string())
    }

    async fn get_token(client: &reqwest::Client, ua: &str, js_url: &str) -> Option<String> {
        let request = Self::new_request(ua, js_url);
        let resp = client.execute(request.try_clone().unwrap()).await.unwrap();
        if resp.status().as_u16() != 200 {
            return None;
        }
        let regex = regex::Regex::new(TOKEN_REGEX).unwrap();
        let body = resp.text().await.unwrap();
        if !regex.is_match(body.as_str()) {
            return None;
        }
        let capture = regex.captures(body.as_str()).unwrap();
        Some(capture.get(1).unwrap().as_str().to_string())
    }

    fn new_request(ua: &str, url: &str) -> reqwest::Request {
        let mut request = reqwest::Request::new(Method::GET, reqwest::Url::parse(url).unwrap());
        let headers = request.headers_mut();
        headers.insert("User-Agent", HeaderValue::from_str(ua).unwrap());
        request
    }
}

impl Drop for Crawler {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.abort()
        }
    }
}
