use crate::Result;
use crate::device::Device;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Debounce {
    Ms0 = 0,
    Ms1 = 1,
    Ms2 = 2,
    Ms4 = 4,
    #[default]
    Ms8 = 8,
    Ms15 = 15,
    Ms20 = 20,
}

impl TryFrom<u8> for Debounce {
    type Error = crate::MadRError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Debounce::Ms0),
            1 => Ok(Debounce::Ms1),
            2 => Ok(Debounce::Ms2),
            4 => Ok(Debounce::Ms4),
            8 => Ok(Debounce::Ms8),
            15 => Ok(Debounce::Ms15),
            20 => Ok(Debounce::Ms20),
            _ => Err(crate::MadRError::InvalidDebounceSetting(format!(
                "Invalid debounce value: {}. Must be one of: 0, 1, 2, 4, 8, 15, 20",
                value
            ))),
        }
    }
}

fn get_debounce_report(debounce: Debounce) -> Vec<u8> {
    let debounce_ms = debounce as u8;
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

/// Apply debounce time
pub fn apply_setting(device: &Device, debounce: Debounce) -> Result<()> {
    let report = get_debounce_report(debounce);
    device.send_feature_report(&report)?;

    Ok(())
}
