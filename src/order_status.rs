use crate::utils;
use crate::GeminiClient;
use serde::Deserialize;
use serde_json::json;

#[derive(Deserialize, Debug, Default)]
pub struct OrderStatus {
    pub order_id: String,
    pub id: Option<String>,
    pub client_order_id: Option<String>,
    pub symbol: String,
    pub exchange: String,
    pub avg_execution_price: String,
    pub side: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub timestamp: String,
    pub timestampms: u64,
    pub is_live: bool,
    pub is_cancelled: bool,
    pub reason: Option<String>,
    pub is_hidden: bool,
    pub was_forced: bool,
    pub executed_amount: String,
    pub remaining_amount: String,
    pub options: Vec<String>,
    pub price: String,
    pub original_amount: String,
}

#[derive(Deserialize, Debug)]
pub struct PastTrade {
    price: String,
    amount: String,
    timestamp: u64,
    timestampms: u64,
    #[serde(rename = "type")]
    type_: String,
    aggressor: bool,
    fee_currency: String,
    fee_amount: String,
    tid: u64,
    order_id: String,
    exchange: String,
    is_auction_fill: bool,
}

pub trait OrderStatusAPI {
    fn order_status(
        &self,
        order_id: Option<u64>,
        client_order_id: Option<&str>,
    ) -> Option<OrderStatus>;

    fn get_active_orders(&self) -> Option<Vec<OrderStatus>>;

    fn get_past_trades(&self, symbol: &str) -> Option<Vec<PastTrade>>;
}

impl OrderStatusAPI for GeminiClient {
    fn order_status(
        &self,
        order_id: Option<u64>,
        client_order_id: Option<&str>,
    ) -> Option<OrderStatus> {
        let endpoint = "/v1/order/status";
        if order_id.is_some() && client_order_id.is_some() {
            println!("order_status must specify either order_id OR client_order_id, not both");
        }
        let payload = match order_id.is_some() {
            true => {
                json!({
                "request"         : endpoint,
                "nonce"           : utils::get_nonce(),
                "order_id"        : order_id,
                 })
            }
            false => {
                json!({
                "request"         : endpoint,
                "nonce"           : utils::get_nonce(),
                "client_order_id" : client_order_id,
                 })
            }
        };
        utils::post_payload(&self.url, endpoint, &payload, &self.api_key, &self.api_sec)
    }

    fn get_active_orders(&self) -> Option<Vec<OrderStatus>> {
        utils::post_standard_payload(&self.url, "/v1/orders", &self.api_key, &self.api_sec)
    }

    fn get_past_trades(&self, symbol: &str) -> Option<Vec<PastTrade>> {
        let endpoint = "/v1/mytrades";
        let payload = json!({
           "request" : endpoint,
           "nonce"   : utils::get_nonce(),
           "symbol"  : symbol,
        });
        utils::post_payload(&self.url, endpoint, &payload, &self.api_key, &self.api_sec)
    }
}
