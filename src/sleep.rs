/*
30 s

first outgoing packet:
unsigned char bytes[] = {0x8, 0x7, 0x0, 0x0, 0xa9, 0xa, 0x4, 0x51, 0x1, 0x54, 0x3, 0x52, 0x0, 0x55, 0x0, 0x55, 0xea};


second outgoing packet:
unsigned char bytes[] = {0x8, 0x7, 0x0, 0x0, 0xb5, 0x6, 0x1, 0x54, 0x3, 0x52, 0x0, 0x55, 0x0, 0x0, 0x0, 0x0, 0x8c};
*/

/*
1 min

first outgoing packet:
unsigned char bytes[] = {0x8, 0x7, 0x0, 0x0, 0xa9, 0xa, 0x4, 0x51, 0x1, 0x54, 0x6, 0x4f, 0x0, 0x55, 0x0, 0x55, 0xea};

second outgoing packet:
unsigned char bytes[] = {0x8, 0x7, 0x0, 0x0, 0xb5, 0x6, 0x1, 0x54, 0x6, 0x4f, 0x0, 0x55, 0x0, 0x0, 0x0, 0x0, 0x8c};
*/

/*
2 min

first outgoing packet:
unsigned char bytes[] = {0x8, 0x7, 0x0, 0x0, 0xa9, 0xa, 0x4, 0x51, 0x1, 0x54, 0xc, 0x49, 0x0, 0x55, 0x0, 0x55, 0xea};

second outgoing packet:
unsigned char bytes[] = {0x8, 0x7, 0x0, 0x0, 0xb5, 0x6, 0x1, 0x54, 0xc, 0x49, 0x0, 0x55, 0x0, 0x0, 0x0, 0x0, 0x8c};
*/

// 3, 5, 20, 25, 30 are rest of the values in web app.

use crate::device::Device;
use std::thread;
use std::time::Duration;

// tells the mouse to go to sleep after X minutes of inactivity
// the packet expects a magic byte in the 11th spot representing tens of seconds
// and the 12th spot is a checksum byte which is 0x55 minus the tens of seconds byte
// so for 30 seconds, we send 3
// for 1 minute, we send 6 etc.
pub fn get_sleep_packet(tens_of_seconds: u8) -> Vec<u8> {
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

// the second packet confirms the sleep time
// the 9th spot is the same tens of seconds byte as before
// the 10th spot is again a checksum byte which is just 0x55 minus the tens of seconds byte
pub fn get_second_packet(tens_of_seconds: u8) -> Vec<u8> {
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
pub fn apply_setting(device: &Device, time_str: &str) -> Result<(), String> {
    let tens_of_seconds: u8 = match time_str {
        "30s" => 3,
        "1m" => 6,
        "2m" => 12,
        "3m" => 18,
        "5m" => 30,
        "20m" => 120,
        "25m" => 150,
        "30m" => 180,
        _ => unreachable!(),
    };

    let packet1 = get_sleep_packet(tens_of_seconds);
    device
        .send_feature_report(&packet1)
        .map_err(|e| format!("Failed to send sleep command: {}", e))?;

    thread::sleep(Duration::from_millis(200));

    let packet2 = get_second_packet(tens_of_seconds);
    device
        .send_feature_report(&packet2)
        .map_err(|e| format!("Failed to send sleep confirmation: {}", e))?;

    println!("Set sleep timeout to {}", time_str);
    Ok(())
}
