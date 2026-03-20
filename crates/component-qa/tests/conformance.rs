use component_qa::{describe_payload, handle_message};

#[test]
fn describe_mentions_world() {
    let payload = describe_payload();
    let json: serde_json::Value = serde_json::from_str(&payload).expect("describe should be json");
    assert_eq!(
        json["component"]["world"],
        "greentic:component/component@0.6.0"
    );
    assert_eq!(json["component"]["self_describing"], true);
}

#[test]
fn describe_version_matches_cargo_pkg_version() {
    let payload = describe_payload();
    let json: serde_json::Value = serde_json::from_str(&payload).expect("describe should be json");
    assert_eq!(
        json["component"]["version"],
        serde_json::Value::String(env!("CARGO_PKG_VERSION").to_string())
    );
}

#[test]
fn handle_echoes_input() {
    let response = handle_message("invoke", "ping");
    assert!(response.contains("ping"));
}
