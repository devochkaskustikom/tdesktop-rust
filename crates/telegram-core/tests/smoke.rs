use telegram_core::ClientId;

#[test]
fn tdlib_creates_client() {
    let client = ClientId::create();
    assert!(client.as_i32() > 0, "client id: {}", client.as_i32());
}
