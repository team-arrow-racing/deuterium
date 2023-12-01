pub enum State {
    Idle,
    Enable,
    Precharge,
    Running,
    Configuration,
}

impl Default for State {
    fn default() -> Self {
        Self::Idle
    }
}
