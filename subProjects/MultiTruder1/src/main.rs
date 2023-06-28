use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, AUTHORIZATION};
use serde::{Serialize, Deserialize};
use serde_json::{json, Value};
use tokio::runtime::Runtime;

#[derive(Serialize)]
struct RequestBody {
    phone: String,
    national_code: String,
}

#[derive(Deserialize)]
struct ResponseBody {
    token: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://myurl.com";
    let token = "my-token";
    let proxy_address = "http://192.168.43.208:8080";

    let mut rt = Runtime::new()?;
    rt.block_on(async {
        // Create the proxy
        let proxy = reqwest::Proxy::https(proxy_address)?;

        // Create the client with the proxy
        let client = reqwest::ClientBuilder::new()
            .proxy(proxy)
            .build()?;

        // Create the request body as a JSON object
        let body = RequestBody {
            phone: String::from("0987654321"),
            national_code: String::from("1234567890"),
        };
        let json_body = json!({
            "phone": body.phone,
            "national_code": body.national_code,
        });

        // Set the headers for the request
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json; charset=utf-8"));
        headers.insert(AUTHORIZATION, format!("Bearer {}", token).parse().unwrap());
        headers.insert(reqwest::header::ACCEPT, HeaderValue::from_static("application/json, text/plain, */*"));

        // Send the request and parse the response
        let response = client.post(url)
            .headers(headers)
            .json(&json_body)
            .send()
            .await?;

        let response_text = response.text().await?;
        println!("{}", response_text);
        let response_json: Value = serde_json::from_str(&response_text)?;

        if let Some(token) = response_json.get("token") {
            if let Some(token_str) = token.as_str() {
                println!("{}", token_str);
            } else {
                println!("token is not a string");
            }
        } else {
            println!("token not found in response");
        }

        Ok(())
    })
}