mod cpu;
use cpu::{Chip_8, State};

use std::env;
use std::time::{Duration, Instant};

// TODO - The accumulators aren't working

fn main() -> Result<(), String> {
    // Instantiate Sdl and cpu
    let sdl_context = sdl2::init()?;
    let mut cpu = Chip_8::build(&sdl_context);

    // Read in arguments
    let args:Vec<String> = env::args().collect();

    // Read ROM into memory
    match cpu.read_into_memory(&args[1]) {
        Ok(_) => {},
        Err(e) => panic!("Failed to read rom {} into memory. {}.", &args[1], e),
    };

    // Instantiate timers and accumulators
    let mut curr_time;
    let mut last_time = Instant::now();
    let mut accumulator_60hz = 0.0;
    let mut accumulator_500hz = 0.0;

    let mut k = 1;
    loop {
        // Calculate the change in time
        curr_time = Instant::now();
        let mut change = (curr_time - last_time).as_secs_f64() * 1000.0;

        // Set 5000 ms cap
        if change > 5000.0 {
            change = 5000.0;
        }

        // Update timers and accumulators
        last_time = curr_time;
        accumulator_60hz += change;
        accumulator_500hz += change;


        // Execute instructions and read user input
        // at 500 Hz
        while accumulator_500hz >= 2.000 {
            println!("{}", k);

            cpu.fetch_decode_execute();
            cpu.keyboard.read_user_input();
            accumulator_500hz -= 2.0;

            k += 1;
        }

        // Decrement audio and delay timers
        // at 60 Hz
        while accumulator_60hz >= 16.667 {
            cpu.decrement_timers();
            accumulator_60hz -= 16.667;
        }
    }

    
}