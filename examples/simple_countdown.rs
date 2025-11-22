//! Simple countdown timer example
//!
//! This example demonstrates using the emom CountdownTimer library
//! in a standalone context (not within the EMOM web app).
//!
//! Note: This example is for documentation purposes and cannot be run
//! in a standard Rust environment as it requires WebAssembly.

use emom::countdown_timer::{CountdownTimer, TimerConfig};

#[allow(dead_code)]
fn example_basic_timer() {
    // Create a timer that ticks every 100ms
    let config = TimerConfig::default();

    let timer = CountdownTimer::new(config, |ticks| {
        println!("Elapsed: {} tenths of a second", ticks);
    });

    timer.start();
    // Timer will now tick every 100ms, calling the callback
    // In a real WASM app, this would integrate with your UI framework
}

#[allow(dead_code)]
fn example_countdown_from_duration() {
    use std::cell::RefCell;
    use std::rc::Rc;

    let remaining = Rc::new(RefCell::new(600)); // 60 seconds in tenths

    let remaining_clone = Rc::clone(&remaining);
    let timer = CountdownTimer::new(TimerConfig::default(), move |_ticks| {
        let mut rem = remaining_clone.borrow_mut();
        if *rem > 0 {
            *rem -= 1;
            println!("Remaining: {}.{} seconds", *rem / 10, *rem % 10);
        }
    });

    timer.start();
}

#[allow(dead_code)]
fn example_custom_config() {
    let config = TimerConfig {
        interval_ms: 100,        // Tick every 100ms
        sync_interval_ticks: 10, // Sync with wall clock every 10 ticks (1 second)
        sync_threshold_ticks: 1, // Correct if drift exceeds 1 tick (100ms)
    };

    let timer = CountdownTimer::new(config, |ticks| {
        println!("Tick {}", ticks);
    });

    timer.start();
}

fn main() {
    println!("This example demonstrates the emom CountdownTimer API.");
    println!("See the function bodies for usage examples.");
    println!("\nNote: This library is designed for WebAssembly environments.");
    println!("To use it in your project, add to Cargo.toml:");
    println!("  emom = {{ version = \"1.0\" }}");
}
