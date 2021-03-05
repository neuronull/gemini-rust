mod fee_volume;
mod order_placer;
mod order_status;
mod public;
mod utils;
pub use fee_volume::FeeVolumeAPI;
pub use order_placer::OrderPlacerAPI;
pub use order_status::OrderStatus;
pub use order_status::OrderStatusAPI;
pub use public::PublicAPI;
pub use public::SymbolDetail;
pub use public::Ticker;

pub struct GeminiClient {
    pub url: String,
    pub api_key: String,
    pub api_sec: String,
}

impl GeminiClient {
    pub fn new(url: &str, api_key: &str, api_sec: &str) -> GeminiClient {
        GeminiClient {
            url: url.to_string(),
            api_key: api_key.to_owned(),
            api_sec: api_sec.to_owned(),
        }
    }
    pub fn new_exchange(api_key: &str, api_sec: &str) -> GeminiClient {
        GeminiClient {
            url: "https://api.gemini.com".to_owned(),
            api_key: api_key.to_owned(),
            api_sec: api_sec.to_owned(),
        }
    }
    pub fn new_sandbox(api_key: &str, api_sec: &str) -> GeminiClient {
        GeminiClient {
            url: "https://api.sandbox.gemini.com".to_owned(),
            api_key: api_key.to_owned(),
            api_sec: api_sec.to_owned(),
        }
    }
}
