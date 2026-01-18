use crate::device::Device;
use crate::{Result, VxeError};

#[derive(Debug)]
pub struct Battery {
    pub percentage: u8,
    pub voltage_mv: u16,
    pub is_charging: bool,
}

fn parse_battery_report(data: &[u8]) -> Result<Battery> {
    if data.len() != 17 || data[0] != 0x08 || data[1] != 0x04 {
        return Err(VxeError::InvalidBatteryFormat);
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

pub fn get_battery_info(device: &Device) -> Result<Battery> {
    let mut packet = [0u8; 17];
    packet[0] = 0x08;
    packet[1] = 0x04;
    packet[16] = 0x55 - (0x08 + packet[1]);

    device.write(&packet)?;

    let mut buf = [0u8; 256];
    let size = device.read_timeout(&mut buf, 20)?;

    let data = &buf[..size];
    let battery = parse_battery_report(data)?;

    Ok(battery)
}
