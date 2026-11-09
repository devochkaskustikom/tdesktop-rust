//! Telegram protocol layer built on TDLib.

/// Identifier of a TDLib client instance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ClientId(i32);

impl ClientId {
    /// Creates a new TDLib client.
    #[must_use]
    pub fn create() -> Self {
        Self(tdlib_rs::create_client())
    }

    /// Returns the raw TDLib client identifier.
    #[must_use]
    pub fn as_i32(self) -> i32 {
        self.0
    }
}
