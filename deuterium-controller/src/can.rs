/// Calculate bit timing register based off
pub fn bit_timing(sysclk: u32, bitrate: u32) -> Result<u32, ()> {
    let mut n_quanta = 16;

    loop {
        let prescale_value: u32 = sysclk / (bitrate * n_quanta);

        if prescale_value < 2 {
            n_quanta -= 1;
        } else {
            let brp = prescale_value - 1;
            let ts2 = (n_quanta / 8) - 1;
            let ts1 = (n_quanta - ts2) - 1;
            let sjw = 0;

            let mut reg: u32 = 0;

            reg |= brp;
            reg |= ts1 << 16;
            reg |= ts2 << 20;
            reg |= sjw << 24;

            return Ok(reg);
        }
    }
}
