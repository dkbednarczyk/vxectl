// Sensor settings:
// 0 = basic
// 1 = competitive
// 2 = competitive MAX

use crate::device::Device;
use std::thread;
use std::time::Duration;

pub fn get_magic_packet(sensor_setting: u8) -> Vec<u8> {
    vec![
        0x08,
        0x07,
        0x00,
        0x00,
        0xb5,
        0x06,
        0x00,
        0x55,
        0x06,
        0x4f,
        sensor_setting,                      // sensor setting byte
        0x55u8.wrapping_sub(sensor_setting), // checksum byte
        0x00,
        0x00,
        0x00,
        0x00,
        0x8c,
    ]
}

/// Apply sensor setting to device
pub fn apply_setting(device: &Device, setting_str: &str) -> Result<(), String> {
    let setting: u8 = match setting_str {
        "basic" => 0,
        "competitive" => 1,
        "max" => 2,
        _ => unreachable!(),
    };

    let packet = get_magic_packet(setting);
    device
        .send_feature_report(&packet)
        .map_err(|e| format!("Failed to send sensor command: {}", e))?;

    println!("Set sensor setting to {}", setting_str);
    thread::sleep(Duration::from_millis(200));
    Ok(())
}
