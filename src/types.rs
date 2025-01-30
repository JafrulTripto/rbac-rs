#[cfg(feature = "uuid-id")]
use uuid::Uuid;

#[cfg(feature = "incremental-id")]
pub type Id = i32;

#[cfg(feature = "uuid-id")]
pub type Id = Uuid;

