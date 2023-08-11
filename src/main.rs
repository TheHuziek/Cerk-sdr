#![no_std]
#![no_main]
mod ili9488;

use embedded_graphics_core::{prelude::{DrawTarget, RgbColor}, pixelcolor::Rgb565};
// use embedded_graphics_core::prelude::RgbColor;
use ili9488::Ili9488;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;
use display_interface_spi::SPIInterface;
use stm32h7xx_hal::{pac, prelude::*, pwr::PwrExt, rcc::RccExt, spi};
// use embedded_graphics::pixelcolor::Bgr565;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = pac::Peripherals::take().unwrap();

    let pwr = dp.PWR.constrain();
    let pwrcfg = pwr.freeze();

    let rcc = dp.RCC.constrain();
    let ccdr = rcc
    .sys_ck(96.MHz())
    .pll1_q_ck(48.MHz())
    .freeze(pwrcfg, &dp.SYSCFG);

    let gpioa = dp.GPIOA.split(ccdr.peripheral.GPIOA);
    let gpiob = dp.GPIOB.split(ccdr.peripheral.GPIOB);
    let gpioc = dp.GPIOC.split(ccdr.peripheral.GPIOC);
    let gpiod = dp.GPIOD.split(ccdr.peripheral.GPIOD);

    let sck = gpioc.pc10.into_alternate();
    let miso = gpioc.pc11.into_alternate();
    let mosi = gpioc.pc12.into_alternate();
    let mut led = gpiob.pb8.into_push_pull_output();
    let dc = gpiod.pd0.into_push_pull_output();
    let rst = gpiod.pd1.into_push_pull_output();
    let cs = gpioa.pa15.into_push_pull_output();
    let spi = dp.SPI3.spi(
        (sck, miso, mosi),
        spi::MODE_0,
        3.MHz(),
        ccdr.peripheral.SPI3,
        &ccdr.clocks,
    );
    // Get the delay provider.
    let mut delay = cp.SYST.delay(ccdr.clocks);
    led.set_high();
    let iface = SPIInterface::new(spi, dc, cs);

    let mut display = Ili9488::new(iface, rst, &mut delay);
    display.clear(Rgb565::BLUE).unwrap();
    loop {}
}
