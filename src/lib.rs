use std::time::Duration;

pub use crate::translator::Translator;

pub mod language_type;
mod translator;
mod crawler;
mod credential;
mod translation;
mod runtime;

const UA: &str = "Mozilla/5.0 (X11; Linux x86_64; rv:89.0) Gecko/20100101 Firefox/89.0";

pub fn default_translator() -> Translator {
    Translator::new(None, Duration::from_secs(10), UA.to_string())
}