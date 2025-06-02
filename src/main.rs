#![no_std]
#![no_main]

use panic_halt as _;
use rp235x_hal as hal;
use cortex_m;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::OutputPin;

// The linker will place this boot block at the start of our program image.
// Note: For RP2350 we need to use the IMAGE_DEF instead of BOOT2
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

#[hal::entry]
fn main() -> ! {
    // Grab our singleton objects
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let _core = cortex_m::Peripherals::take().unwrap();

    // Set up the watchdog driver - needed by the clock setup code
    let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);

    // Configure the clocks
    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000u32, // 12 MHz crystal on Pico 2W
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    // Set up the SIO
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins up according to their function on this particular board
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // Configure the LED pin as an output
    // On Pico 2W, the LED is connected to GPIO 25
    let mut led_pin = pins.gpio25.into_push_pull_output();

    // Set up a delay provider using Timer0
    let mut delay = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // Blink the LED
    loop {
        led_pin.set_high().unwrap();
        delay.delay_ms(500);
        led_pin.set_low().unwrap();
        delay.delay_ms(500);
    }
}