use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use ic_cdk::api::management_canister::http_request::HttpHeader;
use ic_cdk::println;

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[derive(Debug, serde::Deserialize)]
pub struct Response {
    translation_text: String
}

#[ic_cdk::update]
async fn translate(text: String) -> Result<String, String> {
    let token = "";
    let arg = CanisterHttpRequestArgument {
        url: "https://api-inference.huggingface.co/models/google-t5/t5-base".to_string(),
        max_response_bytes: None,
        method: HttpMethod::POST,
        headers: vec![
            HttpHeader {
                name: "Authorization".to_string(),
                value: format!("Bearer {}", token).to_string(),
            }
        ],
        body: Some(format!(r#"{{"inputs": "{}"}}"#, text).into()),
        transform: None,
    };
    let res = http_request(
        arg,
        (1_603_112_800 + text.len() * 400).try_into().unwrap()
    ).await.map_err(|error| format!("Error while querying data. Status: {:?}, Error: {}", error.0, error.1))?;

    let formated_res: (Response,) = serde_json::from_slice(&res.0.body).map_err(|error| format!("Error while parsing data. Error: {}", error))?;
    Ok(formated_res.0.translation_text)
}


