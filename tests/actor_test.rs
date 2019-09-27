extern crate augorama;

use augorama::au::msg::AuTelemetry;

#[test]
fn actor_messaging_works() {
    let t = AuTelemetry::default();
    assert_eq!(t.name, "measurement".to_string());
    assert_eq!(t.value, 0.0);
}
