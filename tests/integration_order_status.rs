use geminent::{GeminiClient, OrderPlacerAPI, OrderStatusAPI};
mod cfg;
use cfg::Cfg;

static BTCUSD: &str = "btcusd";

fn client() -> GeminiClient {
    let cfg = Cfg::new().unwrap();
    GeminiClient::new_sandbox(&cfg.api.key, &cfg.api.sec)
}

#[test]
fn test_order_status() {
    let client = client();

    let amount = 0.25;
    let price = 1.0;
    let client_order_id = "test_order_placer_0";

    let order_status = client
        .new_order(
            BTCUSD,
            amount,
            price,
            "buy",
            "exchange limit",
            client_order_id,
            &vec!["maker-or-cancel"],
        )
        .expect("error with request");

    println!("{:?}", order_status);

    assert_eq!(
        order_status.client_order_id.unwrap(),
        client_order_id,
        "should be client_order_id"
    );
    assert_eq!(order_status.symbol, BTCUSD, "should be btcusd");
    assert_eq!(
        order_status.type_, "exchange limit",
        "should be the type we specified"
    );
    assert!(order_status.timestampms > 0, "timestamp should be > 0");

    let oid : u64 = order_status.order_id.parse().unwrap();
    let testing_status = client
        .order_status(Some(oid), None)
        .expect("error with request");

    println!("{:?}", testing_status);

    assert_eq!(
        order_status.timestampms, testing_status.timestampms,
        "timestampms should be identical"
    );

    let active_orders = client.get_active_orders().expect("error with request");
    assert!(active_orders.len() > 0, "should be an active order");

    let past_trades = client.get_past_trades(BTCUSD).expect("error with request");
    assert!(past_trades.len() != 0, "should be a past trade");
}
