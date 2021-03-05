use hmac_sha512::sha384::HMAC;
use serde::de;
use serde_json::{json, Value};
use std::time::{SystemTime, UNIX_EPOCH};
use ureq::{Request, Response};

pub fn try_deserialize<T: de::DeserializeOwned>(resp: Response) -> Option<T> {
    if resp.ok() {
        //let j = resp.into_string();
        //println!("resp: {:?}", j);
        Some(resp.into_json_deserialize::<T>().unwrap())
    } else {
        println!("error {}: {}", resp.status(), resp.into_string().unwrap());
        None
    }
}

pub fn get_nonce() -> u64 {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("time physics");
    since_the_epoch.as_millis() as u64
}

pub fn post_payload<T: de::DeserializeOwned>(
    url: &str,
    endpoint: &str,
    payload: &Value,
    api_key: &str,
    api_sec: &str,
) -> Option<T> {
    let mut request = ureq::post(&format!("{}{}", url, endpoint).to_owned());
    send(&mut request, &payload, api_key, api_sec)
}

pub fn post_standard_payload<T: de::DeserializeOwned>(
    url: &str,
    endpoint: &str,
    api_key: &str,
    api_sec: &str,
) -> Option<T> {
    let payload = json!({
       "request"         : endpoint,
       "nonce"           : get_nonce(),
    });
    post_payload(url, endpoint, &payload, api_key, api_sec)
}

pub fn send<T: de::DeserializeOwned>(
    request: &mut Request,
    payload: &Value,
    api_key: &str,
    api_sec: &str,
) -> Option<T> {
    set_header(request, &payload, api_key, api_sec);
    try_deserialize::<T>(request.call())
}

pub fn set_header(request: &mut Request, payload: &Value, api_key: &str, api_sec: &str) {
    let encoded_payload = payload.to_string();
    //println!("encoded_payload: {}", encoded_payload);
    let b64 = base64::encode(encoded_payload);
    //println!("b64: {}", b64);

    let signature = HMAC::mac(&b64, api_sec);
    let signature_str = hex::encode(signature);
    //println!("signature: {}", signature_str);

    request
        .set("Content-Type", "text/plain")
        .set("Content-Length", "0")
        .set("X-GEMINI-APIKEY", api_key)
        .set("X-GEMINI-PAYLOAD", &b64)
        .set("X-GEMINI-SIGNATURE", &signature_str)
        .set("Cache-Control", "no-cache");
}
