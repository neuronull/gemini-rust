use crate::order_status::OrderStatus;
use crate::utils;
use crate::GeminiClient;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug)]
pub struct CancelSessionDetails {
    #[serde(rename = "cancelledOrders")]
    pub cancelled_orders: Vec<u64>,
    #[serde(rename = "cancelRejects")]
    pub cancel_rejects: Vec<u64>,
}

#[derive(Deserialize, Debug)]
pub struct CancelResult {
    pub result: String,
    pub details: CancelSessionDetails,
}

pub trait OrderPlacerAPI {
    fn new_order(
        &self,
        symbol: &str,
        amount: f32,
        price: f32,
        side: &str,
        type_: &str,
        client_order_id: &str,
        options: &Vec<&str>,
    ) -> Option<OrderStatus>;

    fn cancel_order(&self, order_id: u64) -> Option<OrderStatus>;
    fn cancel_session(&self) -> Option<CancelResult>;
    fn cancel_all(&self) -> Option<CancelResult>;
}

impl OrderPlacerAPI for GeminiClient {
    fn new_order(
        &self,
        symbol: &str,
        amount: f32,
        price: f32,
        side: &str,
        type_: &str,
        client_order_id: &str,
        options: &Vec<&str>,
    ) -> Option<OrderStatus> {
        let endpoint = "/v1/order/new";
        let payload = json!({
           "request"           : endpoint,
           "nonce"             : utils::get_nonce(),
           "client_order_id"   : client_order_id,
           "symbol"            : symbol,
           "amount"            : format!("{:.6}", amount),
           "price"             : format!("{:.2}", price),
           "side"              : side,
           "type"              : type_,
           "options"           : options,
        });
        utils::post_payload(&self.url, endpoint, &payload, &self.api_key, &self.api_sec)
    }

    fn cancel_order(&self, order_id: u64) -> Option<OrderStatus> {
        let endpoint = "/v1/order/cancel";
        let payload = json!({
            "request"  : endpoint,
            "order_id" : order_id,
            "nonce"    : utils::get_nonce(),
        });
        utils::post_payload(&self.url, endpoint, &payload, &self.api_key, &self.api_sec)
    }

    fn cancel_session(&self) -> Option<CancelResult> {
        let endpoint = "/v1/order/cancel/session";
        let payload = json!({
            "request"  : endpoint,
            "nonce"    : utils::get_nonce(),
        });
        utils::post_payload(&self.url, endpoint, &payload, &self.api_key, &self.api_sec)
    }

    fn cancel_all(&self) -> Option<CancelResult> {
        let endpoint = "/v1/order/cancel/all";
        let payload = json!({
            "request"  : endpoint,
            "nonce"    : utils::get_nonce(),
        });
        utils::post_payload(&self.url, endpoint, &payload, &self.api_key, &self.api_sec)
    }
}
