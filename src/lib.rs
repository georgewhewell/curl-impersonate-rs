mod client;
mod error;

pub use client::CurlClient;
pub use error::CurlError;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    pub async fn test_main() {
        let mut client = CurlClient::chrome();
        client.set_header("Accept", "*/*");
        client.set_header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:108.0) Gecko/20100101 Firefox/108.0",
        );
        let response = client.get("https://httpbin.org/get").await.expect("get");
        println!("Response: {:?}", response);
    }
}
