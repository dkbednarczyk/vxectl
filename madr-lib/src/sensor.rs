use crate::device::Device;
use crate::{Result, VxeError};

#[derive(Debug)]
pub struct Sensor {
    setting: u8,
}

impl Sensor {
    pub fn from_bytes(data: &[u8]) -> Option<Sensor> {
        if data.len() < 17 || data[0] != 0x08 || data[1] != 0x08 {
            return None;
        }

        Some(Self { setting: data[10] })
    }

    pub fn name_from_setting(&self) -> &str {
        match self.setting {
            0 => "basic",
            1 => "competitive",
            2 => "max",
            _ => "unknown",
        }
    }

    pub fn setting_from_name(name: &str) -> Option<u8> {
        match name {
            "basic" => Some(0),
            "competitive" => Some(1),
            "max" => Some(2),
            _ => None,
        }
    }
}

pub fn get_sensor_info(device: &Device) -> Result<Sensor> {
    let mut packet = [0u8; 17];
    packet[0] = 0x08;
    packet[1] = 0x08;
    packet[4] = 0xB5;
    packet[5] = 0x06;
    packet[16] = 0x8a;

    device.write(&packet)?;

    let mut buf = [0u8; 17];
    device.read_timeout(&mut buf, 20)?;

    Sensor::from_bytes(&buf).ok_or(VxeError::InvalidSensorFormat)
}

pub fn get_magic_packet(sensor_setting: u8) -> Vec<u8> {
    vec![
        0x08,
        0x07,
        0x00,
        0x00,
        0xb5,                                // magic
        0x06,                                // bytes
        0x00,           // works with either 00 or 01? after factory reset 00 is correct though
        0x55,           // 55 - prev
        0x06,           // magic
        0x4f,           // magic
        sensor_setting, // sensor setting byte
        0x55u8.wrapping_sub(sensor_setting), // checksum byte
        0x00,
        0x00,
        0x00,
        0x00,
        0x8c,
    ]
}

/// Apply sensor setting to device
pub fn apply_setting(device: &Device, setting_str: &str) -> Result<()> {
    let setting = Sensor::setting_from_name(setting_str)
        .ok_or_else(|| VxeError::InvalidSensorSetting(setting_str.into()))?;

    let packet = get_magic_packet(setting);
    device.send_feature_report(&packet)?;

    Ok(())
}
