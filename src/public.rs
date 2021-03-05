use crate::utils;
use crate::GeminiClient;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct SymbolDetail {
    pub symbol: String,
    pub base_currency: String,
    pub quote_currency: String,
    pub tick_size: u8,
    pub quote_increment: u8,
    pub min_order_size: String,
    pub status: String,
}

#[derive(Deserialize, Debug)]
pub struct Volume {
    pub quantity: String,
    pub price: String,
    pub timestamp: u64,
}

#[derive(Deserialize, Debug)]
pub struct Ticker {
    pub symbol: String,
    pub open: String,
    pub high: String,
    pub low: String,
    pub close: String,
    pub changes: Vec<String>,
    pub bid: String,
    pub ask: String,
}

#[derive(Debug)]
pub struct Candle {
    pub timestamp: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: f64,
}

#[derive(Deserialize, Debug)]
pub struct Order {
    pub price: String,
    pub amount: String,
    pub timestamp: String, // DO NOT USE, INVALID
}

#[derive(Deserialize, Debug)]
pub struct OrderBook {
    pub bids: Vec<Order>,
    pub asks: Vec<Order>,
}

#[derive(Deserialize, Debug)]
pub struct Trade {
    pub timestamp: u64,
    pub timestampms: u64,
    pub tid: u64,
    pub price: String,
    pub amount: String,
    pub exchange: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Deserialize, Debug)]
pub struct Auction {
    pub closed_until_ms: Option<u64>,
    pub last_auction_price: Option<String>,
    pub last_auction_quantity: Option<String>,
    pub last_highest_bid_price: Option<String>,
    pub last_lowest_ask_price: Option<String>,
    pub last_collar_price: Option<String>,
    pub most_recent_indicative_price: Option<String>,
    pub most_recent_indicative_quantity: Option<String>,
    pub most_recent_highest_bid_price: Option<String>,
    pub most_recent_lowest_ask_price: Option<String>,
    pub most_recent_collar_price: Option<String>,
    pub next_update_ms: Option<u64>,
    pub next_auction_ms: u64,
}

#[derive(Deserialize, Debug)]
pub struct AuctionHistory {
    pub auction_id: u64,
    pub auction_price: Option<String>,
    pub auction_quantity: Option<String>,
    pub eid: u64,
    pub highest_bid_price: Option<String>,
    pub lowest_ask_price: Option<String>,
    pub collar_price: String,
    pub auction_result: String,
    pub timestamp: u64,
    pub timestampms: u64,
    pub event_type: String,
}

#[derive(Deserialize, Debug)]
pub struct PriceFeed {
    pub pair: String,
    pub price: String,
    #[serde(rename = "percentChange24h")]
    pub percent_change_24h: String,
}

pub trait PublicAPI {
    fn symbols(&self) -> Option<Vec<String>>;

    fn symbol_detail(&self, symbol: &str) -> Option<SymbolDetail>;
    fn ticker(&self, symbol: &str) -> Option<Ticker>;

    fn candles(&self, symbol: &str, time_frame: &str) -> Option<Vec<Candle>>;

    fn order_book(
        &self,
        symbol: &str,
        _limit_asks: Option<u16>,
        _limit_bids: Option<u16>,
    ) -> Option<OrderBook>;

    fn trades(
        &self,
        symbol: &str,
        _limit_trades: Option<u16>,
        _timestamp: Option<u64>,
        _include_breaks: Option<bool>,
    ) -> Option<Vec<Trade>>;

    fn auction(&self, symbol: &str) -> Option<Auction>;

    fn auction_history(
        &self,
        symbol: &str,
        _limit_auction_results: Option<u16>,
        _include_indicative: Option<bool>,
    ) -> Option<Vec<AuctionHistory>>;

    fn price_feed(&self) -> Option<Vec<PriceFeed>>;
}

impl PublicAPI for GeminiClient {
    fn symbols(&self) -> Option<Vec<String>> {
        utils::try_deserialize::<Vec<String>>(
            ureq::get(&format!("{}/v1/symbols", self.url).to_owned()).call(),
        )
    }

    fn symbol_detail(&self, symbol: &str) -> Option<SymbolDetail> {
        utils::try_deserialize::<SymbolDetail>(
            ureq::get(&format!("{}/v1/symbols/details/{}", self.url, symbol).to_owned()).call(),
        )
    }

    fn ticker(&self, symbol: &str) -> Option<Ticker> {
        utils::try_deserialize::<Ticker>(
            ureq::get(&format!("{}/v2/ticker/{}", self.url, symbol).to_owned()).call(),
        )
    }

    fn candles(&self, symbol: &str, time_frame: &str) -> Option<Vec<Candle>> {
        let resp =
            ureq::get(&format!("{}/v2/candles/{}/{}", self.url, symbol, time_frame).to_owned())
                .call()
                .into_json();
        if !resp.is_ok() {
            return None;
        }
        let t = resp.unwrap();
        let arr = t.as_array().unwrap();
        if arr.len() == 0 {
            return None;
        }
        let mut candles = vec![];
        for a in arr {
            let c = Candle {
                timestamp: a[0].as_u64().unwrap(),
                open: a[1].as_f64().unwrap(),
                high: a[2].as_f64().unwrap(),
                low: a[3].as_f64().unwrap(),
                close: a[4].as_f64().unwrap(),
                volume: a[5].as_f64().unwrap(),
            };
            candles.push(c);
        }
        Some(candles)
    }

    fn order_book(
        &self,
        symbol: &str,
        _limit_asks: Option<u16>,
        _limit_bids: Option<u16>,
    ) -> Option<OrderBook> {
        utils::try_deserialize::<OrderBook>(
            ureq::get(&format!("{}/v1/book/{}", self.url, symbol).to_owned()).call(),
        )
    }

    fn trades(
        &self,
        symbol: &str,
        _limit_trades: Option<u16>,
        _timestamp: Option<u64>,
        _include_breaks: Option<bool>,
    ) -> Option<Vec<Trade>> {
        utils::try_deserialize::<Vec<Trade>>(
            ureq::get(&format!("{}/v1/trades/{}", self.url, symbol).to_owned()).call(),
        )
    }

    fn auction(&self, symbol: &str) -> Option<Auction> {
        utils::try_deserialize::<Auction>(
            ureq::get(&format!("{}/v1/auction/{}", self.url, symbol).to_owned()).call(),
        )
    }

    fn auction_history(
        &self,
        symbol: &str,
        _limit_auction_results: Option<u16>,
        _include_indicative: Option<bool>,
    ) -> Option<Vec<AuctionHistory>> {
        utils::try_deserialize::<Vec<AuctionHistory>>(
            ureq::get(&format!("{}/v1/auction/{}/history", self.url, symbol).to_owned()).call(),
        )
    }

    fn price_feed(&self) -> Option<Vec<PriceFeed>> {
        utils::try_deserialize::<Vec<PriceFeed>>(
            ureq::get(&format!("{}/v1/pricefeed", self.url).to_owned()).call(),
        )
    }
}
