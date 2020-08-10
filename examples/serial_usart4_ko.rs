#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*, serial};
use core::fmt::Write;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(160.mhz()).freeze(vos, &dp.SYSCFG);

    // Acquire the GPIOB peripheral. This also enables the clock for
    // GPIOB in the RCC register.
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);

    let tx = gpiob.pb9.into_alternate_af8();
    let rx = gpiob.pb8.into_alternate_af8();


    // Configure the serial peripheral.
    let mut serial = dp
        .UART4
        .usart(
            (tx, rx),
            serial::config::Config::default(),
            ccdr.peripheral.UART4,
            &ccdr.clocks,
        )
        .unwrap();

    let (mut tx, mut _rx) = serial.split();    

    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);

    loop {
        writeln!(tx, "Hello, world!").unwrap();
        delay.delay_ms(500_u16);
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
