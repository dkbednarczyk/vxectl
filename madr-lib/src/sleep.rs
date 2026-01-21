use crate::Result;
use crate::device::Device;
use std::time::Duration;

fn get_sleep_report(tens_of_seconds: u8) -> Vec<u8> {
    vec![
        0x08,
        0x07,
        0x00,
        0x00,
        0xA9,
        0x0A,
        0x04,
        0x51,
        0x01,
        0x54,
        tens_of_seconds,
        0x55u8.wrapping_sub(tens_of_seconds),
        0x00,
        0x55,
        0x00,
        0x55,
        0xEA,
    ]
}

fn get_confirmation_report(tens_of_seconds: u8) -> Vec<u8> {
    vec![
        0x08,
        0x07,
        0x00,
        0x00,
        0xB5,
        0x06,
        0x01,
        0x54,
        tens_of_seconds,
        0x55u8.wrapping_sub(tens_of_seconds),
        0x00,
        0x55,
        0x00,
        0x00,
        0x00,
        0x00,
        0x8C,
    ]
}

/// Apply sleep timeout setting to device
pub fn apply_setting(device: &Device, duration: Duration) -> Result<()> {
    let time_ms = duration.as_millis() as u32;
    let tens_of_seconds = (time_ms / 10000) as u8;

    let sleep_pkt = get_sleep_report(tens_of_seconds);
    device.send_feature_report(&sleep_pkt)?;

    let confirmation = get_confirmation_report(tens_of_seconds);
    device.send_feature_report(&confirmation)?;

    Ok(())
}
