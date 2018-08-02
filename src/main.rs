#![no_main]
#![no_std]

extern crate cortex_m;
#[macro_use]
extern crate cortex_m_rt as rt;
extern crate cortex_m_semihosting as sh;
extern crate nucleo_f767zi;
extern crate panic_semihosting;

mod fault_condition;
mod pid;
mod throttle_module;

use core::fmt::Write;
use cortex_m::peripheral::syst::SystClkSource;
use nucleo_f767zi::hal::stm32f7x7;
use rt::ExceptionFrame;
use sh::hio;

entry!(main);

fn main() -> ! {
    let mut stdout = hio::hstdout().unwrap();
    let peripherals = cortex_m::Peripherals::take().unwrap();
    //let peripherals = stm32f7x7::Peripherals::take().unwrap();

    let mut syst = peripherals.SYST;

    // need to fix/add HAL RCC to get full clock speed
    syst.set_clock_source(SystClkSource::Core);
    syst.set_reload(8_000_000); // 1s
    syst.enable_counter();

    loop {
        while !syst.has_wrapped() {}
        writeln!(stdout, "Hello, world!").unwrap();
    }
}

exception!(HardFault, hard_fault);

fn hard_fault(ef: &ExceptionFrame) -> ! {
    panic!("HardFault at {:#?}", ef);
}

exception!(*, default_handler);

fn default_handler(irqn: i16) {
    panic!("Unhandled exception (IRQn = {})", irqn);
}
