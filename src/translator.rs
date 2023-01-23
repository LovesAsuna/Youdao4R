use crate::crawler::Crawler;
use crate::credential::Credential;
use crate::language_type::LanguageType;
use crate::runtime::ModuleRuntime;
use crate::translation::Translation;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{Client, ClientBuilder, RequestBuilder};
use std::cell::RefCell;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc::Receiver;

const API_URL: &str = "https://fanyi.youdao.com/translate_o?smartresult=dict&smartresult=rule";
pub const CLIENT: &str = "fanyideskweb";
const VERSION: &str = "2.1";

pub struct Translator {
    client: Client,
    user_agent: String,
    token: RefCell<String>,
    runtime: ModuleRuntime,
    crawler: RefCell<Receiver<String>>,
}

impl Translator {
    pub fn new(client: Option<Client>, caching_time: Duration, user_agent: String) -> Self {
        let mut client = match client {
            None => {
                let mut builder = ClientBuilder::new();
                builder = builder.cookie_store(true);
                builder.build().unwrap()
            }
            Some(c) => c,
        };
        let runtime = ModuleRuntime::new();
        // fetch cookie
        client = runtime.block_on(async {
            client.get("https://fanyi.youdao.com").send().await.unwrap();
            client
        });
        // fetch token
        let (tx, rx) = tokio::sync::mpsc::channel(1);
        let sender = Arc::new(tx);
        let ua = user_agent.clone();
        runtime.spawn(async move {
            Crawler::new(ua, caching_time, sender).await;
        });
        Self {
            client,
            user_agent,
            token: RefCell::default(),
            runtime,
            crawler: RefCell::new(rx),
        }
    }

    pub fn translate(
        &mut self,
        from: LanguageType,
        to: LanguageType,
        timeout: Duration,
        original_text: &str,
    ) -> Option<String> {
        let task = self.translate_async(from, to, timeout, original_text);
        self.runtime.block_on(task)
    }

    pub async fn translate_async(
        &self,
        from: LanguageType,
        to: LanguageType,
        timeout: Duration,
        original_text: &str,
    ) -> Option<String> {
        *self.token.borrow_mut() = self.crawler.borrow_mut().recv().await.unwrap();
        let builder = self.build_request(from, to, timeout, original_text);
        let request = builder.build().unwrap();
        let response = self.client.execute(request).await.ok();
        match response {
            None => None,
            Some(r) => Self::process_response(r.text().await.ok()),
        }
    }

    fn build_request(
        &self,
        from: LanguageType,
        to: LanguageType,
        timeout: Duration,
        original_text: &str,
    ) -> RequestBuilder {
        let mut params: Vec<(&str, &str)> = Vec::new();
        let original_text = &Self::escape_chars(original_text);
        params.push(("i", original_text));
        params.push(("from", from.0));
        params.push(("to", to.0));
        params.push(("smartresult", "dict"));
        params.push(("client", CLIENT));
        let credential = Credential::of(&self.token.borrow(), original_text);
        params.append(&mut credential.to_params());
        params.push(("doctype", "json"));
        params.push(("version", VERSION));
        params.push(("keyfrom", "fanyi.web"));
        params.push(("action", "FY_BY_CLICKBUTTION"));
        let builder = self.client.post(API_URL).form(&params).timeout(timeout);
        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Type",
            HeaderValue::from_str("application/x-www-form-urlencoded; charset=UTF-8").unwrap(),
        );
        headers.insert(
            "User-Agent",
            HeaderValue::from_str(&self.user_agent).unwrap(),
        );
        headers.insert(
            "Referer",
            HeaderValue::from_str("https://fanyi.youdao.com/").unwrap(),
        );
        headers.insert(
            "Origin",
            HeaderValue::from_str("https://fanyi.youdao.com/").unwrap(),
        );
        builder.headers(headers)
    }

    fn escape_chars(str: &str) -> String {
        let map: Vec<(String, String)> = serde_urlencoded::from_str(str).unwrap();
        map.into_iter().next().unwrap().0
    }

    fn process_response(body: Option<String>) -> Option<String> {
        match body {
            None => None,
            Some(x) => {
                let translation = serde_json::from_str::<Translation>(&x);
                Some(format!("{}", translation.expect("deserialize error")))
            }
        }
    }
}
