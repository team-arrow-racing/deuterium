//! I/O Type Definitions
//!
//! These are put here since they don't change very much.

use crate::hal::{
    can::Can,
    device::{CAN1, CAN2, I2C1},
    gpio::*,
    i2c::I2c,
};

/// CAN Bus
pub type Can1Tx = PA12<Alternate<PushPull, 9>>;
pub type Can1Rx = PA11<Alternate<PushPull, 9>>;
pub type Can1 = Can<CAN1, (Can1Tx, Can1Rx)>;

// todo: add support for CAN2 to stm32l4xx-hal crate
pub type Can2Tx = PB6<Alternate<PushPull, 8>>;
pub type Can2Rx = PB5<Alternate<PushPull, 3>>;
pub type Can2 = Can<CAN2, (Can2Tx, Can2Rx)>;

/// I2C
pub type I2c1Scl = PB6<Alternate<PushPull, 4>>;
pub type I2c1Sda = PB7<Alternate<PushPull, 4>>;
pub type I2c1 = I2c<I2C1, (I2c1Scl, I2c1Sda)>;

/// Status LEDs
pub type LedStatus = PB13<Output<OpenDrain>>;
pub type LedError = PB14<Output<OpenDrain>>;

/// Analog inputs
pub type BatteryVoltageSense = PC0<Analog>;
pub type OutputVoltageSense = PC1<Analog>;
pub type CurrentSense = PC2<Analog>;

/// Fans
pub type Fan1Out = PA0<Alternate<PushPull, 1>>;
pub type Fan1PwmIn = PA1<Alternate<Floating, 1>>;
pub type Fan2Out = PA2<Alternate<PushPull, 1>>;
pub type Fan2PwmIn = PA3<Alternate<Floating, 1>>;
