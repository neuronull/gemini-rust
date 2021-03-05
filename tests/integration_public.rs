use geminent::{GeminiClient, PublicAPI};

static BTCUSD: &str = "btcusd";

fn client() -> GeminiClient {
    GeminiClient::new_sandbox("", "")
}

#[test]
fn test_public_symbols() {
    let client = client();
    let symbols = client.symbols().expect("error with request");
    assert!(symbols.len() > 0, "symbols vector should be > 0 size");
    assert!(
        symbols.contains(&BTCUSD.to_string()),
        "symbols should contain btcusd"
    );
    println!("{:?}", symbols);
}

#[test]
fn test_public_symbol_detail() {
    let client = client();
    let detail = client.symbol_detail(BTCUSD).expect("error with request");
    assert_ne!(detail.status, "", "order status should be populated");
    println!("{:?}", detail);
}

#[test]
fn test_public_ticker() {
    let client = client();
    let ticker = client.ticker(BTCUSD).expect("error with request");
    assert_ne!(ticker.ask, "", "ask should be populated");
    println!("{:?}", ticker);
}

#[test]
fn test_public_candles() {
    let client = client();
    let candles = client.candles(BTCUSD, "15m").expect("error with request");
    assert_ne!(candles.len(), 0, "should be some candles");
    assert_ne!(candles[0].timestamp, 0, "timestamp should be > 0");
    println!("{:?}", candles);
}

#[test]
fn test_public_order_book() {
    let client = client();
    let order_book = client
        .order_book(BTCUSD, None, None)
        .expect("error with request");
    assert_ne!(order_book.bids.len(), 0, "should be some bids");
    assert_ne!(order_book.asks.len(), 0, "should be some asks");
    assert_ne!(order_book.asks[0].price, "", "price error");
    println!("{:?}", order_book);
}

#[test]
fn test_public_trades() {
    let client = client();
    let trades = client
        .trades(BTCUSD, None, None, None)
        .expect("error with request");
    assert_ne!(trades.len(), 0, "should be some trades");
    assert_ne!(trades[0].timestamp, 0, "timestamp should be > 0");
    println!("{:?}", trades);
}

#[test]
fn test_public_auction() {
    let client = client();
    let auction = client.auction(BTCUSD).expect("error with request");
    assert_ne!(auction.next_auction_ms, 0, "next_auction_ms should be > 0");
    println!("{:?}", auction);
}

#[test]
fn test_public_auction_history() {
    let client = client();
    let auction_history = client
        .auction_history(BTCUSD, None, None)
        .expect("error with request");
    assert_ne!(auction_history.len(), 0, "should be some auction");
    assert_ne!(auction_history[0].timestamp, 0, "timestamp should be > 0");
    println!("{:?}", auction_history);
}

#[test]
fn test_public_price_feed() {
    let client = client();
    let price_feed = client.price_feed().expect("error with request");
    assert_ne!(price_feed.len(), 0, "should be some price feeds");
    assert_ne!(price_feed[0].price, "", "price should be populated");
    println!("{:?}", price_feed);
}
