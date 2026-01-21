use crate::device::Device;
use crate::{MadRError, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Battery {
    percentage: u8,
    voltage_mv: u16,
    is_charging: bool,
}

impl Battery {
    /// Read battery status from the device
    pub fn read(device: &Device) -> Result<Self> {
        let mut report = [0u8; 17];
        report[0] = 0x08;
        report[1] = 0x04;
        report[16] = 0x55 - (0x08 + report[1]);

        device.write(&report)?;

        let mut buf = [0u8; 256];
        let size = device.read_timeout(&mut buf, 20)?;

        let data = &buf[..size];
        Self::parse_report(data)
    }

    fn parse_report(data: &[u8]) -> Result<Self> {
        if data.len() < 17 || data[0] != 0x08 || data[1] != 0x04 {
            return Err(MadRError::InvalidBatteryFormat);
        }

        let percentage = data[6];
        let is_charging = data[7] == 0x01;
        let voltage_mv = u16::from_be_bytes([data[8], data[9]]);

        Ok(Battery {
            percentage,
            voltage_mv,
            is_charging,
        })
    }

    /// Battery percentage (0-100)
    pub fn percentage(&self) -> u8 {
        self.percentage
    }

    /// Voltage in millivolts
    pub fn voltage(&self) -> u16 {
        self.voltage_mv
    }

    /// Is the battery currently charging
    pub fn is_charging(&self) -> bool {
        self.is_charging
    }
}
