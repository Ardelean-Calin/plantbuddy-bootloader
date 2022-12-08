#![no_std]
#![no_main]
#![macro_use]

use cortex_m_rt::{entry, exception};
#[cfg(feature = "defmt")]
use defmt::info;
#[cfg(feature = "defmt")]
use defmt_rtt as _;

#[cfg(feature = "panic-probe")]
use panic_probe as _;

use embassy_boot_nrf::*;
use embassy_nrf::nvmc::Nvmc;

#[entry]
fn main() -> ! {
    let p = embassy_nrf::init(Default::default());

    // Uncomment this if you are debugging the bootloader with debugger/RTT attached,
    // as it prevents a hard fault when accessing flash 'too early' after boot.
    /*
        for i in 0..10000000 {
            cortex_m::asm::nop();
        }
    */

    let mut bl = BootLoader::default();
    let start = bl.prepare(&mut SingleFlashConfig::new(&mut BootFlash::<_, 4096>::new(
        Nvmc::new(p.NVMC), // Note: I can wrap this inside a WatchdogFlash that also creates a watchdog
                           // timer. Useful if I want to feed the timer inside the main application every
                           // so-often! Resets if timer was not feeded!
    )));
    unsafe { bl.load(start) }
}

#[no_mangle]
#[cfg_attr(target_os = "none", link_section = ".HardFault.user")]
unsafe extern "C" fn HardFault() {
    cortex_m::peripheral::SCB::sys_reset();
}

#[exception]
unsafe fn DefaultHandler(_: i16) -> ! {
    const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32;
    let irqn = core::ptr::read_volatile(SCB_ICSR) as u8 as i16 - 16;

    panic!("DefaultHandler #{:?}", irqn);
}

#[cfg(not(feature = "debug"))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    cortex_m::asm::udf();
}
