//! I/O Type Definitions

use crate::hal::{can::Can, device::CAN1, gpio::*};

/// CAN Bus
pub type CanTx = PA12<Alternate<PushPull, 9>>;
pub type CanRx = PA11<Alternate<PushPull, 9>>;
pub type Can1 = Can<CAN1, (CanTx, CanRx)>;

/// Status LEDs
pub type LedStatus = PB13<Output<OpenDrain>>;
pub type LedError = PB14<Output<OpenDrain>>;
