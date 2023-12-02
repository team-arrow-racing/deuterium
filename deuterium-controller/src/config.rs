use serde::Deserialize;

/// Configuration File
#[derive(Deserialize, Default)]
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
#[derive(Deserialize, Default)]
pub struct Comms {
    /// CAN bus communications
    pub canbus: CommsCanbus,
    /// Modbus communications
    pub modbus: CommsModbus,
}

/// CAN bus communications
#[derive(Deserialize)]
pub struct CommsCanbus {
    /// Bitrate (bit/s)
    pub bitrate: u32,
    /// Base identifier address
    pub id: u16,
}

impl Default for CommsCanbus {
    fn default() -> Self {
        Self {
            bitrate: 500_000,
            id: 0x600,
        }
    }
}

/// Modbus communications
#[derive(Deserialize)]
pub struct CommsModbus {
    /// Serial baud rate (baud)
    pub baudrate: u32,
    /// Slave ID
    pub id: u8,
}

impl Default for CommsModbus {
    fn default() -> Self {
        Self {
            baudrate: 115_200,
            id: 10,
        }
    }
}

/// Battery
#[derive(Deserialize)]
pub struct Battery {
    /// Minimum voltage limit (V)
    pub voltage_min: f32,
    /// Maximum voltage limit (V)
    pub voltage_max: f32,
    /// Minimum temperature limit (°C)
    pub temperature_min: f32,
    /// Maximum temperature limit (°C)
    pub temperature_max: f32,
    /// Maximum current limit (A)
    pub current_max: f32,
}

impl Default for Battery {
    fn default() -> Self {
        Self {
            voltage_min: 95.0,
            voltage_max: 165.0,
            temperature_min: 0.0,
            temperature_max: 50.0,
            current_max: 100.0,
        }
    }
}

/// Cell
#[derive(Deserialize)]
pub struct Cell {
    /// Minimum voltage limit (V)
    pub voltage_min: f32,
    /// Maximum voltage limit (V)
    pub voltage_max: f32,
    /// Minimum temperature limit (°C)
    pub temperature_min: f32,
    /// Maximum temperature limit (°C)
    pub temperature_max: f32,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            voltage_min: 2.5,
            voltage_max: 4.2,
            temperature_min: 0.0,
            temperature_max: 45.0,
        }
    }
}

/// Precharge
#[derive(Deserialize)]
pub struct Precharge {
    /// Precharge timeout (seconds)
    pub timeout: f32,
    /// Precharge resistor maximum temperature limit (°C)
    pub temperature_max: f32,
}

impl Default for Precharge {
    fn default() -> Self {
        Self {
            timeout: 5.0,
            temperature_max: 95.0,
        }
    }
}
