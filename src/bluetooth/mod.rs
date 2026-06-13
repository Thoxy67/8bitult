//! Bluetooth plumbing: powering the adapter, connecting, and reading
//! notifications.
//!
//! - [`power`]: makes sure the adapter is on and scanning (Linux only).
//! - [`connect`]: connects with retries.
//! - [`characteristic`]: finds the writable characteristic and waits on
//!   notifications.

pub mod characteristic;
pub mod connect;
pub mod power;
