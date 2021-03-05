use geminent::{GeminiClient, OrderPlacerAPI};
mod cfg;
use cfg::Cfg;

static BTCUSD: &str = "btcusd";

fn client() -> GeminiClient {
    let cfg = Cfg::new().unwrap();
    GeminiClient::new_sandbox(&cfg.api.key, &cfg.api.sec)
}

#[test]
fn test_order_placer_new_order() {
    let client = client();

    let amount = 0.25;
    let price = 1.00;
    let order_id = "test_order_placer_0";

    let order_status = client
        .new_order(
            BTCUSD,
            amount,
            price,
            "buy",
            "exchange limit",
            order_id,
            &vec!["maker-or-cancel"],
        )
        .expect("error with request");

    println!("{:?}", order_status);

    assert_eq!(
        order_status.client_order_id.unwrap(),
        order_id,
        "should be order_id"
    );
    assert_eq!(order_status.symbol, BTCUSD, "should be btcusd");
    assert_eq!(
        order_status.type_, "exchange limit",
        "should be the type we specified"
    );
    assert!(order_status.timestampms > 0, "timestamp should be > 0");
}

#[test]
fn test_order_placer_cancel_session() {
    let client = client();
    let cancel_result = client.cancel_session().expect("error with request");
    println!("{:?}", cancel_result);
    assert_eq!(cancel_result.result, "ok", "cancel session should succeed");
}

#[test]
fn test_order_placer_cancel_all() {
    let client = client();
    let cancel_result = client.cancel_all().expect("error with request");
    println!("{:?}", cancel_result);
    assert_eq!(cancel_result.result, "ok", "cancel all should succeed");
}
