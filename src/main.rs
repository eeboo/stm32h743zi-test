//#![deny(warnings)]
#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32h7xx_hal::{pac, prelude::*, serial};

#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take().unwrap();

    // Constrain and Freeze power
    let pwr = dp.PWR.constrain();
    let vos = pwr.freeze();

    // Constrain and Freeze clock
    let rcc = dp.RCC.constrain();
    let ccdr = rcc.sys_ck(160.mhz()).freeze(vos, &dp.SYSCFG);

    // Acquire the GPIOC peripheral. This also enables the clock for
    // GPIOC in the RCC register.
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);

    let tx = gpioc.pc10.into_alternate_af7();
    let rx = gpioc.pc11.into_alternate_af7();


    // Configure the serial peripheral.
    let mut serial = dp
        .USART3
        .usart(
            (tx, rx),
            serial::config::Config::default(),
            ccdr.peripheral.USART3,
            &ccdr.clocks,
        )
        .unwrap();

    loop {
        serial.write(122);
    }
}

#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
