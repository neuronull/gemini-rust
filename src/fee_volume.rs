use crate::utils;
use crate::GeminiClient;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct NotationalVolume {
    pub date: String,
    pub notional_volume: f32,
}

#[derive(Deserialize, Debug)]
pub struct PriceVolume {
    pub web_maker_fee_bps: u16,
    pub web_taker_fee_bps: u16,
    pub web_auction_fee_bps: u16,
    pub api_maker_fee_bps: u16,
    pub api_taker_fee_bps: u16,
    pub api_auction_fee_bps: u16,
    pub fix_maker_fee_bps: u16,
    pub fix_taker_fee_bps: u16,
    pub fix_auction_fee_bps: u16,
    pub block_maker_fee_bps: u16,
    pub block_taker_fee_bps: u16,
    pub notional_30d_volume: f32,
    pub last_updated_ms: u64,
    pub date: String,
    pub notional_1d_volume: Vec<NotationalVolume>,
}

#[derive(Deserialize, Debug)]
pub struct TradeVolume {
    pub symbol: String,
    pub base_currency: String,
    pub notional_currency: String,
    pub data_date: String,
    pub total_volume_base: f32,
    pub maker_buy_sell_ratio: f32,
    pub buy_maker_base: f32,
    pub buy_maker_notional: f32,
    pub buy_maker_count: f32,
    pub sell_maker_base: f32,
    pub sell_maker_notional: f32,
    pub sell_maker_count: f32,
    pub buy_taker_base: f32,
    pub buy_taker_notional: f32,
    pub buy_taker_count: f32,
    pub sell_taker_base: f32,
    pub sell_taker_notional: f32,
    pub sell_taker_count: f32,
}

pub trait FeeVolumeAPI {
    fn get_notional_volume(&self) -> Option<PriceVolume>;

    fn get_trade_volume(&self) -> Option<Vec<Vec<TradeVolume>>>;
}

impl FeeVolumeAPI for GeminiClient {
    fn get_notional_volume(&self) -> Option<PriceVolume> {
        utils::post_standard_payload(
            &self.url,
            "/v1/notionalvolume",
            &self.api_key,
            &self.api_sec,
        )
    }

    fn get_trade_volume(&self) -> Option<Vec<Vec<TradeVolume>>> {
        utils::post_standard_payload(&self.url, "/v1/tradevolume", &self.api_key, &self.api_sec)
    }
}
