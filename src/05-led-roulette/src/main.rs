#![deny(unsafe_code)]
#![no_main]
#![no_std]

use aux5::{entry, prelude::*, Delay, Leds};

#[entry]
fn main() -> ! {
    let (mut delay, mut leds): (Delay, Leds) = aux5::init();

    let half_period = 50_u16;
    let mut index = 0;
    let mut interval = 0;

    loop {
        let next_index = (index + 1) % leds.len();

        match interval {
            0 => {
                leds[index].on();
                leds[next_index].on();
            },
            1 => {
                leds[index].off();
                index = next_index;
            },
            _ => {},
        }
        interval = (interval + 1) % 2;
        delay.delay_ms(half_period);
    }
}
