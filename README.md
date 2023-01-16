curl-impersonate
================

Wraps [curl-impersonate](https://github.com/lwthiker/curl-impersonate) with Rust async wrapper

Usage
-----

Make sure `curl-impersonate-ff`, `curl-impersonate-chrome` etc are in your `$PATH`

    let mut client = CurlClient::chrome();
    client.set_header("Accept", "*/*");
    client.set_header(
        "User-Agent",
        "Mozilla/5.0 (X11; Linux x86_64; rv:108.0) Gecko/20100101 Firefox/108.0",
    );
    let response = client.get("https://httpbin.org/get").await.expect("get");
    println!("Response: {:?}", response);

Disclaimer
----------

written by GPT. fuck cloudflare