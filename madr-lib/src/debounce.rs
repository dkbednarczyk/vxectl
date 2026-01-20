use crate::Result;
use crate::device::Device;

fn get_debounce_report(debounce_ms: u8) -> Vec<u8> {
    vec![
        0x08,
        0x07,
        0x00,
        0x00,
        0xA9,
        0x0A,
        debounce_ms,
        0x55u8.wrapping_sub(debounce_ms),
        0x01,
        0x54,
        0x06,
        0x4F,
        0x00,
        0x55,
        0x00,
        0x55,
        0xEA,
    ]
}

pub fn apply_setting(device: &Device, debounce_str: &str) -> Result<()> {
    let debounce_val: u8 = debounce_str.parse()?;

    let report = get_debounce_report(debounce_val);
    device.send_feature_report(&report)?;

    Ok(())
}
