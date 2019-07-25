// use std::error::Error;
// use std::thread;
// use std::time::Duration;

// use rppal::gpio::Gpio;

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
// const GPIO_LED: u8 = 14;

// fn main() -> Result<(), Box<dyn Error>> {
//     // Retrieve the GPIO pin and configure it as an output.
//     let mut pin = Gpio::new()?.get(GPIO_LED)?.into_output();

//     loop {
//         pin.toggle();
//         thread::sleep(Duration::from_millis(500));
//     }
// }

use std::env;
extern crate time;
use time::PreciseTime;

fn main() {
    // allows user to enter a command line arg (number)
    let FIB_SEED: u32 = u32::from_str_radix(env::args().nth(1).unwrap().as_str(), 10).unwrap();

    let start = PreciseTime::now();
    let x: u32 = fibonacci(FIB_SEED);
    let end = PreciseTime::now();

    println!("{:?} for crunching fibonnaci sequence with seed of: {}", start.to(end), FIB_SEED);
    println!("{} recursive calls", x);
}

fn fibonacci(n: u32) -> u32 {
  match n {
    0 => 1,
    1 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2)
  }
}