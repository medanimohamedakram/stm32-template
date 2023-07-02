#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::delay;
use cortex_m_rt::entry;
use stm32f1::stm32f103;

#[entry]
fn main() -> ! {

    let peripherals = stm32f103::Peripherals::take().unwrap();
    let core_peripherals = stm32f103::CorePeripherals::take().unwrap();
    let mut delay = delay::Delay::new(core_peripherals.SYST, 72_000_000);

    let rcc = &peripherals.RCC;
    let gpioa = &peripherals.GPIOA;
    let gpiob = &peripherals.GPIOB;
    let adc1 = &peripherals.ADC1;
    
    rcc.apb2enr.modify(|_, w| w.iopaen().enabled()
                                .iopben().enabled()
                                .adc1en().enabled());
    rcc.cfgr.modify(|_, w| w.adcpre().div6());

    gpiob.crl.modify(|_, w| w.mode0().input().cnf1().push_pull());

    gpioa.crl.write(|w| unsafe { w.bits(0x33333333) });
    gpioa.crh.write(|w| unsafe { w.bits(0x44443333) });

    adc1.cr2.modify(|_, w| w.adon().enabled());
    delay.delay_ms(100);

    adc1.sqr1.write(|w| w.l().bits(0));
    adc1.sqr3.write(|w| unsafe { w.sq1().bits(8) });

    loop {
        adc1.cr2.modify(|_, w| w.adon().enabled());
        while adc1.sr.read().eoc().is_not_complete() {}
        let data = adc1.dr.read().data().bits();
        let output = 2u32.pow(data as u32/340)-1;
        gpioa.odr.write(|w| unsafe { w.bits(output as u32) });

    }
}
