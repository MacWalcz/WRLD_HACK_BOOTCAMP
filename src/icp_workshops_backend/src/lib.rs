use ic_cdk::api::management_canister::http_request::http_request;
use ic_cdk::api::management_canister::http_request::HttpMethod;
use ic_cdk::api::management_canister::http_request::CanisterHttpRequestArgument;
use ic_cdk::api::management_canister::http_request::HttpHeader;
use serde_json::{Value, json};

#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[ic_cdk::query]
fn calculate(a: i32, b: i32, operator: String) -> String {
    let result = match operator.as_str() {
        "+" => Some(a+b),
        "-" => Some(a-b),
        "*" => Some(a*b),
        "/" => if b != 0 {Some(a/b)} else {None},
        "%" => if b != 0 {Some(a%b)} else {None},
        _ => None
    };

    match result {
        Some(value) => format!("Result: {}", value),
        None => "Wrong operator or division by zero".to_string()
    }


}

#[ic_cdk::update]
async fn translate(text: String) -> String {
    let token = "";
    let (response,) = http_request(
        CanisterHttpRequestArgument {
            url: "https://api-inference.huggingface.co/models/google-t5/t5-base".to_string(),
            max_response_bytes: None,
            method: HttpMethod::POST,
            headers: vec![
                HttpHeader{
                    name: "Authorization".to_string(),
                    value: format!("Bearer {}",token).to_string()
                },
                HttpHeader {
                    name: "Content-Type".to_string(),
                    value: "application/json".to_string(),
                }

            ],
            body: Some(format!(r#"{{"inputs": "{}"}}"#,text).into()),
            transform: None
        },
        20_849_972_000u128 + text.len() as u128 * 5200u128
     
       

    ).await.unwrap();
    
    let body = String::from_utf8(response.body).unwrap();

    

    let json: Value = serde_json::from_str(&body).unwrap();
    
   
    let translated_text = json[0]["translation_text"]
    .as_str()
    .unwrap();

    format!("Hello, {}!", translated_text)
}