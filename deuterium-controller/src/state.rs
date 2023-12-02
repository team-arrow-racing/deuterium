/// Battery state
#[derive(Default)]
pub enum State {
    #[default]
    Idle,
    Enable,
    Precharge(PrechargeState),
    Running,
    Configuration,
}

/// Precharge state
#[derive(Default)]
pub enum PrechargeState {
    #[default]
    Isolated,
    Negative,
    Charging,
}
