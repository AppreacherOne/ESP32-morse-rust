#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_hal::timer::timg::TimerGroup;
use esp_hal::interrupt::software::SoftwareInterruptControl;

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) {
    // generator version: 1.0.0

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    esp_rtos::start(timg0.timer0, sw_ints.software_interrupt0);


    let buzzer = Output::new(peripherals.GPIO33, Level::Low, OutputConfig::default());

    let text = "rust is the best";

    let wpm = 20;

    spawner.spawn(morse(wpm, text, buzzer).unwrap());
}

#[embassy_executor::task]
async fn morse(speed: u64, text: &'static str, mut buzzer: Output<'static>) {


    let short_beep: u64 = 1200 / speed;
    let long_beep: u64 = 3 * short_beep;

    loop {
        for letter in text.chars() {
            match letter {
                'a' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'b' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'c' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'd' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'e' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'f' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'g' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'h' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'i' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'j' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'k' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'l' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'm' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'n' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'o' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'p' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'q' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'r' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                's' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                't' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'u' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'v' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'w' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'x' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'y' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                'z' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '0' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '1' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '2' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '3' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '4' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '5' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '6' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '7' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '8' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '9' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                '?' => {
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(long_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_high();
                    Timer::after(Duration::from_millis(short_beep)).await;
                    buzzer.set_low();
                    Timer::after(Duration::from_millis(long_beep)).await
                },
                ' ' => {
                    Timer::after(Duration::from_millis(500)).await;
                }
                _ => Timer::after(Duration::from_millis(1)).await
            }
        }
    }
}