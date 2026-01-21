// Performance settings module
// DPI stage and polling rate share the same report structure (0x08 0x07 0x00 0x00 0x00 0x06)
// and can be combined into a single configuration report.

use crate::device::Device;
use crate::{MadRError, Result};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PollingRate {
    Hz125 = 125,
    Hz250 = 250,
    Hz500 = 500,
    Hz1000 = 1000,
    Hz2000 = 2000,
    Hz4000 = 4000,
    Hz8000 = 8000,
}

impl TryFrom<u16> for PollingRate {
    type Error = MadRError;

    fn try_from(value: u16) -> Result<Self> {
        match value {
            125 => Ok(PollingRate::Hz125),
            250 => Ok(PollingRate::Hz250),
            500 => Ok(PollingRate::Hz500),
            1000 => Ok(PollingRate::Hz1000),
            2000 => Ok(PollingRate::Hz2000),
            4000 => Ok(PollingRate::Hz4000),
            8000 => Ok(PollingRate::Hz8000),
            _ => Err(MadRError::InvalidPerformanceSetting(format!(
                "Unsupported polling rate: {}",
                value
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Performance {
    dpi_stage: u8,
    polling_rate: PollingRate,
}

impl Performance {
    pub fn new(dpi_stage: u8, polling_rate: PollingRate) -> Self {
        Self {
            dpi_stage,
            polling_rate,
        }
    }

    pub fn dpi_stage(&self) -> u8 {
        self.dpi_stage
    }

    pub fn polling_rate(&self) -> PollingRate {
        self.polling_rate
    }

    pub fn read(device: &Device) -> Result<Self> {
        let mut report = [0u8; 17];
        report[0] = 0x08;
        report[1] = 0x08;
        report[5] = 0x06;
        report[16] = 0x3f;

        device.write(&report)?;

        let mut buf = [0u8; 17];
        device.read_timeout(&mut buf, 20)?;

        Self::from_bytes(&buf)
    }

    fn from_bytes(data: &[u8]) -> Result<Performance> {
        let dpi_stage = data[10] + 1; // stored as stage - 1
        let polling_rate = match data[6] {
            0x08 => PollingRate::Hz125,
            0x04 => PollingRate::Hz250,
            0x02 => PollingRate::Hz500,
            0x01 => PollingRate::Hz1000,
            0x10 => PollingRate::Hz2000,
            0x20 => PollingRate::Hz4000,
            0x40 => PollingRate::Hz8000,
            _ => {
                return Err(MadRError::InvalidPerformanceSetting(
                    "Unsupported polling rate".into(),
                ));
            }
        };

        Ok(Self {
            dpi_stage,
            polling_rate,
        })
    }
}

fn make_combined_report(dpi_stage: u8, rate: PollingRate) -> Result<Vec<u8>> {
    let rate_byte: u8 = match rate {
        PollingRate::Hz125 => 0x08,
        PollingRate::Hz250 => 0x04,
        PollingRate::Hz500 => 0x02,
        PollingRate::Hz1000 => 0x01,
        PollingRate::Hz2000 => 0x10,
        PollingRate::Hz4000 => 0x20,
        PollingRate::Hz8000 => 0x40,
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
pub fn apply_settings(device: &Device, settings: &Performance) -> Result<()> {
    let report = make_combined_report(settings.dpi_stage, settings.polling_rate)?;
    device.send_feature_report(&report)?;

    Ok(())
}
