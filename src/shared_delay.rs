use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::DelayMs;

pub struct SharedTimer {
    systick_freq_hz: u32,
    systick: SYST,
}

impl SharedTimer {
    pub fn new(mut systick: SYST, core_freq_hz: u32) -> Self {
        // Configure SysTick to count down from its max value continuously
        systick.set_reload(u32::MAX);
        systick.clear_current();
        systick.enable_counter();

        Self {
            systick_freq_hz: core_freq_hz,
            systick,
        }
    }

    pub fn now(&self) -> u32 {
        // SysTick counts down, so we invert it for elapsed time comparison
        u32::MAX - SYST::get_current()
    }

    pub fn release(self) -> SYST {
        self.systick
    }
}

pub struct DelayTimer<'a> {
    timer: &'a SharedTimer,
}

impl<'a> DelayTimer<'a> {
    pub fn new(timer: &'a SharedTimer) -> Self {
        Self { timer }
    }
}

impl DelayMs<u32> for DelayTimer<'_> {
    fn delay_ms(&mut self, ms: u32) {
        let ticks = ms * (self.timer.systick_freq_hz / 1000);
        let start = self.timer.now();
        while self.timer.now().wrapping_sub(start) < ticks {}
    }
}

impl DelayMs<u16> for DelayTimer<'_> {
    fn delay_ms(&mut self, ms: u16) {
        let ticks = ms as u32 * (self.timer.systick_freq_hz / 1000);
        let start = self.timer.now();
        while self.timer.now().wrapping_sub(start) < ticks {}
    }
}

impl DelayMs<u8> for DelayTimer<'_> {
    fn delay_ms(&mut self, ms: u8) {
        let ticks = ms as u32 * (self.timer.systick_freq_hz / 1000);
        let start = self.timer.now();
        while self.timer.now().wrapping_sub(start) < ticks {}
    }
}
