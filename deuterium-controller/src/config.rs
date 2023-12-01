use serde::Deserialize;

/// Configuration File
#[derive(Deserialize)]
pub struct Config {
    /// Communications
    pub comms: Comms,
    /// Battery
    pub battery: Battery,
    /// Cell
    pub cell: Cell,
    /// Precharge
    pub precharge: Precharge,
}

/// Communications
#[derive(Deserialize)]
pub struct Comms {
    /// CAN bus communications
    pub canbus: CommsCanbus,
    /// Modbus communications
    pub modbus: CommsModbus,
}

/// CAN bus communications
#[derive(Deserialize)]
pub struct CommsCanbus {
    /// Bitrate
    pub bitrate: u32,
    /// Base identifier address
    pub id: u16,
}

/// Modbus communications
#[derive(Deserialize)]
pub struct CommsModbus {
    /// Serial baud rate
    pub baudrate: u32,
    /// Slave ID
    pub id: u8,
}

/// Battery
#[derive(Deserialize)]
pub struct Battery {
    /// Minimum voltage limit
    pub voltage_min: f32,
    /// Maximum voltage limit
    pub voltage_max: f32,
    /// Minimum temperature limit
    pub temperature_min: f32,
    /// Maximum temperature limit
    pub temperature_max: f32,
    /// Maximum current limit
    pub current_max: f32,
}

/// Cell
#[derive(Deserialize)]
pub struct Cell {
    /// Minimum voltage limit
    pub voltage_min: f32,
    /// Maximum voltage limit
    pub voltage_max: f32,
    /// Minimum temperature limit
    pub temperature_min: f32,
    /// Maximum temperature limit
    pub temperature_max: f32,
}

/// Precharge
#[derive(Deserialize)]
pub struct Precharge {
    pub timeout: u32,
}
