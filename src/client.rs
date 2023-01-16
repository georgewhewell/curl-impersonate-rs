/*
Provides an interface to curl-impersonate-{ff,chrome} binaries.

async_process::Command is used to execute the binary and communicate with it.

Can request compressed responses (via accept-encoding header)
Compressed responses are detected by looking at the content-encoding header.

flate2 crate is used to decompress the response body.
*/

use crate::error::CurlError;
use async_process::Command;
use flate2::read::GzDecoder;
use std::{collections::HashMap, io::Read};

pub enum Browser {
    Firefox,
    Chrome,
    Custom(String),
}

impl Browser {
    fn to_string(&self) -> String {
        match self {
            Browser::Firefox => "curl-impersonate-ff".to_string(),
            Browser::Chrome => "curl-impersonate-chrome".to_string(),
            Browser::Custom(s) => s.to_string(),
        }
    }
}

pub struct CurlClient {
    binary: String,
    headers: HashMap<String, String>,
    accept_encoding: Option<String>,
}

impl Default for CurlClient {
    fn default() -> CurlClient {
        CurlClient::new(Browser::Chrome)
    }
}

fn decompress(body: Vec<u8>) -> Result<Vec<u8>, CurlError> {
    let mut decoder = GzDecoder::new(&body[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;
    Ok(decompressed)
}

impl CurlClient {
    pub fn chrome() -> CurlClient {
        CurlClient::new(Browser::Chrome)
    }

    pub fn firefox() -> CurlClient {
        CurlClient::new(Browser::Firefox)
    }

    pub fn new(browser: Browser) -> CurlClient {
        CurlClient {
            binary: browser.to_string(),
            headers: HashMap::new(),
            accept_encoding: Some("gzip".to_string()),
        }
    }

    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    pub fn set_accept_encoding(&mut self, value: &str) {
        self.accept_encoding = Some(value.to_string());
    }

    async fn get_bytes(&self, url: &str) -> Result<Vec<u8>, CurlError> {
        let mut cmd = Command::new(&self.binary);
        for (key, value) in &self.headers {
            cmd.arg("-H");
            cmd.arg(format!("{}: {}", key, value));
        }
        if let Some(accept_encoding) = &self.accept_encoding {
            cmd.arg("-H");
            cmd.arg(format!("accept-encoding: {}", accept_encoding));
        }
        cmd.arg(url);
        dbg!(&cmd);
        let output = cmd.output().await?;
        Ok(output.stdout)
    }

    pub async fn get(&self, url: &str) -> Result<String, CurlError> {
        let bytes = self.get_bytes(url).await?;
        let s = match decompress(bytes.clone()) {
            Ok(decompressed) => String::from_utf8(decompressed)?,
            Err(e) => String::from_utf8(bytes)?,
        };
        Ok(s)
    }
}
