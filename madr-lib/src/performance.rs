// Performance settings module
// DPI stage and polling rate share the same report structure (0x08 0x07 0x00 0x00 0x00 0x06)
// and can be combined into a single configuration report.

use crate::device::Device;
use crate::{Result, MadRError};

pub fn get_combined_report(dpi_stage: u8, rate: u16) -> Result<Vec<u8>> {
    let rate_byte: u8 = match rate {
        125 => 0x08,
        250 => 0x04,
        500 => 0x02,
        1000 => 0x01,
        // wireless only
        2000 => 0x10,
        4000 => 0x20,
        8000 => 0x40,
        _ => return Err(MadRError::InvalidPerformanceSetting(
            "Unsupported polling rate".into(),
        ))?,
    };

    Ok(vec![
        0x08,
        0x07,
        0x00,
        0x00,
        0x00,
        0x06,
        rate_byte,                      // byte 6: polling rate
        0x55u8.wrapping_sub(rate_byte), // byte 7: polling rate checksum
        0x04,
        0x51,                               // bytes 8-9: magic bits
        dpi_stage - 1,                      // byte 10: DPI stage
        0x55u8.wrapping_sub(dpi_stage - 1), // byte 11: DPI stage checksum
        0x00,
        0x00,
        0x00,
        0x00,
        0x41, // bytes 12-16: trailer
    ])
}

/// Apply performance settings to device
pub fn apply_settings(
    device: &Device,
    dpi_stage: u8,
    polling_rate: u16,
) -> Result<()> {
    let report = get_combined_report(dpi_stage, polling_rate)?;
    device.send_feature_report(&report)?;

    Ok(())
}
