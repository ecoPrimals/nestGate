//! API request handlers

pub mod auth;
pub mod health;
pub mod status;
pub mod zfs;
pub mod hardware_tuning;

pub use health::*;
pub use zfs::*;
pub use hardware_tuning::*;
