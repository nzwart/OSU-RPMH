use cortex_m::peripheral::SYST;
use embedded_hal::blocking::delay::DelayNs;

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
        u32::MAX - self.systick.current()
    }

    pub fn release(self) -> SYST {
        self.systick
    }
}

pub struct DelayTimer<'a> {
    timer: &'a SharedTimer,
}

impl<'a> DelayNs for DelayTimer<'a> {
    fn delay_ns(&mut self, ns: u32) {
        let ticks = ns / 1000 * (self.timer.systick_freq_hz / 1_000_000);
        let start = self.timer.now();
        while self.timer.now().wrapping_sub(start) < ticks {}
    }
}

