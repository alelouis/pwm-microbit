#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use panic_rtt_target as _;

use microbit::hal::{pac::Peripherals, gpio, pwm, gpio::Level};
use microbit::hal::prelude::*;

/*
PWM and micro:bit example code.
Code companion for : alelouis.eu/blog/pwm-microbit
 */


#[entry]
fn main() -> ! {
    // Init RRT
    rtt_init_print!();

    // Get ownership of peripherals
    let board = Peripherals::take().expect("Couldn't initialize board.");

    // Configuring output pin
    let gpio = gpio::p0::Parts::new(board.P0);
    let pwm_pin = gpio.p0_02.into_push_pull_output(Level::Low).degrade();

    // Configuring output pin
    let pwm_motor = pwm::Pwm::new(board.PWM0);
    pwm_motor.set_output_pin(pwm::Channel::C0, pwm_pin);
    pwm_motor.set_prescaler(pwm::Prescaler::Div32);
    pwm_motor.set_max_duty(10_000_u16);
    pwm_motor.loop_inf();

    // Print the period in order to check the configuration
    rprintln!("{}", pwm_motor.period().0);

    // Define duty cycle
    let mut duty = 650_u16;
    pwm_motor.set_duty_off(pwm::Channel::C0, duty);

    // Run indefinitely
    loop {}
}