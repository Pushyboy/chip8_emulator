mod cpu;
use cpu::{Chip_8, State};

mod display;
use display::Display;

mod keyboard;
use keyboard::Keyboard;

// use CPU::Chip_8;
use std::env;
use std::time::{Duration, Instant};


fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;

    // Move Display to Chip8

    let mut keyboard = Keyboard::new(&sdl_context).unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let args:Vec<String> = env::args().collect();

    let mut cpu = Chip_8::build();
    cpu.read_into_memory(&args[1]);

    let mut currTime;
    let mut lastTime = Instant::now();
    let mut accumulator_60hz = 0.0;
    let mut accumulator_500hz = 0.0;

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        currTime = Instant::now();
        let mut change = (currTime - lastTime).as_secs_f64() / 1000.0;

        // Set 5000 ms cap
        if change > 5000.0 {
            change = 5000.0;
        }

        // If awaiting for keypress
        if cpu.state == State::INACTIVE && keyboard.is_pressed {
            // Have to store it in vx
        }

        // Update timers and accumulators
        lastTime = currTime;
        accumulator_60hz += change;
        accumulator_500hz += change;

        while accumulator_500hz >= 2.000 {
            cpu.fetch_decode_execute();
            accumulator_500hz -= 2.0;
        }

        while accumulator_60hz >= 16.667 {
            cpu.decrement_timers();
            accumulator_60hz -= 16.67;
        }
    }

    
}