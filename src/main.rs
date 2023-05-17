#![no_std]
#![no_main]
#![feature(c_variadic)]

use core::ffi::c_char;

use esp_backtrace as _;
use esp_println::println;
use hal::{clock::ClockControl, peripherals::Peripherals, prelude::*, timer::TimerGroup, Rtc};

extern "C" {
    fn somethingsomething();
}

#[no_mangle]
unsafe extern "C" fn write_log(lvl: u32, a: *const c_char, b: *const c_char, mut args: ...) {
    println!(
        "{} {:?} {:?}",
        lvl,
        core::ffi::CStr::from_ptr(a),
        core::ffi::CStr::from_ptr(b)
    );
    for i in 1..=9 {
        let x = args.arg::<u32>();
        println!("{i} = {x}");
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let mut system = peripherals.DPORT.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    // Disable the RTC and TIMG watchdog timers
    let mut rtc = Rtc::new(peripherals.RTC_CNTL);
    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt0 = timer_group0.wdt;
    let timer_group1 = TimerGroup::new(
        peripherals.TIMG1,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut wdt1 = timer_group1.wdt;
    rtc.rwdt.disable();
    wdt0.disable();
    wdt1.disable();

    unsafe {
        somethingsomething();
    }

    loop {}
}
