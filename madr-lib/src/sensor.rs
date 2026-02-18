use crate::device::Device;
use crate::{MadRError, Result};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SensorMode {
    #[default]
    Basic = 0,
    Competitive = 1,
    Max = 2,
}

impl fmt::Display for SensorMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SensorMode::Basic => write!(f, "basic"),
            SensorMode::Competitive => write!(f, "competitive"),
            SensorMode::Max => write!(f, "max"),
        }
    }
}

impl FromStr for SensorMode {
    type Err = MadRError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "basic" => Ok(SensorMode::Basic),
            "competitive" => Ok(SensorMode::Competitive),
            "max" => Ok(SensorMode::Max),
            _ => Err(MadRError::InvalidSensorSetting(s.into())),
        }
    }
}

impl TryFrom<u8> for SensorMode {
    type Error = MadRError;

    fn try_from(value: u8) -> Result<Self> {
        match value {
            0 => Ok(SensorMode::Basic),
            1 => Ok(SensorMode::Competitive),
            2 => Ok(SensorMode::Max),
            _ => Err(MadRError::InvalidSensorSetting(value.to_string())),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sensor {
    mode: SensorMode,
}

impl Sensor {
    /// Read sensor configuration from device
    pub fn read(device: &Device) -> Result<Self> {
        let mut report = [0u8; 17];
        report[0] = 0x08;
        report[1] = 0x08;
        report[4] = 0xB5;
        report[5] = 0x06;
        report[16] = 0x8a;

        device.write(&report)?;

        let mut buf = [0u8; 17];
        device.read_timeout(&mut buf, 20)?;

        let data = &buf;
        if data.len() < 17 || data[0] != 0x08 || data[1] != 0x08 {
            return Err(MadRError::InvalidSensorFormat);
        }

        let mode = SensorMode::try_from(data[10])?;
        Ok(Self { mode })
    }

    pub fn mode(&self) -> SensorMode {
        self.mode
    }
}

fn get_magic_report(sensor_mode: SensorMode) -> Vec<u8> {
    let setting = sensor_mode as u8;
    vec![
        0x08,
        0x07,
        0x00,
        0x00,
        0xb5,    // magic...
        0x06,    // ...bytes
        0x00,    // works with either 00 or 01? after factory reset 00 is correct though
        0x55,    // 55 - prev
        0x06,    // magic
        0x4f,    // magic
        setting, // sensor setting byte
        0x55u8.wrapping_sub(setting), // checksum byte
        0x00,
        0x00,
        0x00,
        0x00,
        0x8c,
    ]
}

/// Apply sensor setting to device
pub fn apply_setting(device: &Device, mode: SensorMode) -> Result<()> {
    let report = get_magic_report(mode);
    device.send_feature_report(&report)?;

    Ok(())
}
