pub mod battery;
pub mod debounce;
pub mod device;
pub mod dpi;
pub mod performance;
pub mod sensor;
pub mod sleep;

use thiserror::Error;

/// Unified error type for all vxelib operations
#[derive(Error, Debug)]
pub enum MadRError {
    #[error("Failed to initialize HIDAPI: {0}")]
    HidApiInit(#[from] hidapi::HidError),
    #[error("No compatible device found")]
    DeviceNotFound,
    #[error("Invalid battery report format")]
    InvalidBatteryFormat,
    #[error("Invalid sensor report format")]
    InvalidSensorFormat,
    #[error("Invalid sensor setting: {0}")]
    InvalidSensorSetting(String),
    #[error("Invalid sleep timeout: {0}")]
    InvalidSleepTimeout(String),
    #[error("Invalid debounce value: {0}")]
    InvalidDebounce(#[from] std::num::ParseIntError),
    #[error("Invalid DPI setting: {0}")]
    InvalidDpi(String),
    #[error("Invalid RGB value: {0}")]
    InvalidRgb(String),
    #[error("Invalid performance setting: {0}")]
    InvalidPerformanceSetting(String),
}

pub type Result<T> = std::result::Result<T, MadRError>;
