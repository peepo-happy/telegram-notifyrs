use serde_json::json;
use serde_json::{Map, Value};
use ureq;

/// Sends a Telegram message
///
/// Sends the supplied message to the designated chad ID, using the supplied token.
pub fn send_message(
    msg: String,
    token: &str,
    chat_id: i64,
) -> std::result::Result<ureq::Response, ureq::Error> {
    let mut request_body = Map::new();
    request_body.insert("text".to_string(), Value::String(msg));
    request_body.insert("chat_id".to_string(), json!(chat_id));

    let resp = ureq::post(&format!(
        "https://api.telegram.org/bot{token}/sendMessage",
        token = &token
    ))
    .send_json(json!(request_body));
    return resp;
}

// pub fn send_photo(
//     photo: String,
//     caption: String,
//     token: &str,
//     chat_id: i64,
// ) -> std::result::Result<ureq::Response, ureq::Error> {
//     let mut request_body = Map::new();
//     request_body.insert("chat_id".to_string(), json!(chat_id));
//     request_body.insert("photo".to_string(), Value::String(photo));
//     request_body.insert("caption".to_string(), Value::String(caption));
//
//
//     let resp = ureq::post(&format!(
//         "https://api.telegram.org/bot{token}/sendPhoto",
//         token = &token
//     ))
//         .send_json(json!(request_body));
//     return resp;
// }

// pub fn send_media_group(
//     photo: String,
//     caption: String,
//     token: &str,
//     chat_id: i64,
// ) -> std::result::Result<ureq::Response, ureq::Error> {
//     let mut request_body = Map::new();
//     request_body.insert("chat_id".to_string(), json!(chat_id));
//     request_body.insert("photo".to_string(), Value::String(photo));
//     request_body.insert("caption".to_string(), Value::String(caption));
//
//
//     let resp = ureq::post(&format!(
//         "https://api.telegram.org/bot{token}/sendMediaGroup",
//         token = &token
//     ))
//         .send_json(json!(request_body));
//     return resp;
// }