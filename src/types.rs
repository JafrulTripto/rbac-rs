#[cfg(feature = "uuid-id")]
use uuid::Uuid;

#[cfg(feature = "incremental-id")]
pub type Id = i32;

#[cfg(all(feature = "uuid-id", not(feature = "incremental-id")))]
pub type Id = Uuid;

#[cfg(all(feature = "incremental-id", not(feature = "uuid-id")))]
pub type Id = i32;
