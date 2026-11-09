use anyhow::Result;
use telegram_core::ClientId;

fn main() -> Result<()> {
    let client = ClientId::create();
    println!("TDLib client id: {}", client.as_i32());
    Ok(())
}
