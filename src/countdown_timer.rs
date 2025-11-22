//! Simple countdown timer - straightforward implementation

use gloo_timers::callback::Timeout;
use js_sys::Date;
use std::cell::{Cell, RefCell};
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct TimerConfig {
    pub interval_ms: u32,
    pub sync_interval_ticks: usize,
    pub sync_threshold_ticks: usize,
}

impl Default for TimerConfig {
    fn default() -> Self {
        Self {
            interval_ms: 100,
            sync_interval_ticks: 10,
            sync_threshold_ticks: 1,
        }
    }
}

struct TimerState {
    running: bool,
    ticks_elapsed: usize,
    next_tick_time: f64,
    start_time: f64,
}

/// CountdownTimer must be wrapped in Rc for the recursive callback to work
pub struct CountdownTimer<F>
where
    F: Fn(usize) + 'static,
{
    config: TimerConfig,
    state: Rc<RefCell<TimerState>>,
    timeout_handle: Rc<Cell<Option<Timeout>>>,
    on_tick: Rc<F>,
}

impl<F> CountdownTimer<F>
where
    F: Fn(usize) + 'static,
{
    pub fn new(config: TimerConfig, on_tick: F) -> Rc<Self> {
        Rc::new(Self {
            config,
            state: Rc::new(RefCell::new(TimerState {
                running: false,
                ticks_elapsed: 0,
                next_tick_time: 0.0,
                start_time: 0.0,
            })),
            timeout_handle: Rc::new(Cell::new(None)),
            on_tick: Rc::new(on_tick),
        })
    }

    pub fn start(self: &Rc<Self>) {
        let mut state = self.state.borrow_mut();
        if state.running {
            return;
        }

        let now = Date::now();
        state.running = true;
        state.ticks_elapsed = 0;
        state.start_time = now;
        state.next_tick_time = now + self.config.interval_ms as f64;
        drop(state);

        self.schedule_tick();
    }

    pub fn stop(&self) {
        let mut state = self.state.borrow_mut();
        state.running = false;
        drop(state);

        if let Some(handle) = self.timeout_handle.take() {
            handle.cancel();
        }
    }

    pub fn reset(&self) {
        let mut state = self.state.borrow_mut();
        state.running = false;
        state.ticks_elapsed = 0;
        state.next_tick_time = 0.0;
        state.start_time = 0.0;
        drop(state);

        if let Some(handle) = self.timeout_handle.take() {
            handle.cancel();
        }
    }

    pub fn elapsed_ticks(&self) -> usize {
        self.state.borrow().ticks_elapsed
    }

    pub fn is_running(&self) -> bool {
        self.state.borrow().running
    }

    fn schedule_tick(self: &Rc<Self>) {
        let delay = {
            let state = self.state.borrow();
            if !state.running {
                return;
            }
            let now = Date::now();
            (state.next_tick_time - now).max(0.0).round() as u32
        };

        let state_clone = Rc::clone(&self.state);
        let config = self.config;
        let on_tick_clone = Rc::clone(&self.on_tick);
        let timeout_handle_clone = Rc::clone(&self.timeout_handle);
        let timer_clone = Rc::clone(self);

        let handle = Timeout::new(delay, move || {
            timeout_handle_clone.take();

            let mut state = state_clone.borrow_mut();
            if !state.running {
                return;
            }

            let now = Date::now();
            state.ticks_elapsed += 1;
            state.next_tick_time += config.interval_ms as f64;

            if state
                .ticks_elapsed
                .is_multiple_of(config.sync_interval_ticks)
            {
                let elapsed_ms = now - state.start_time;
                let expected_ticks = (elapsed_ms / config.interval_ms as f64).floor() as usize;
                let tick_diff = expected_ticks.abs_diff(state.ticks_elapsed);

                if tick_diff > config.sync_threshold_ticks {
                    state.ticks_elapsed = expected_ticks;
                    state.next_tick_time = state.start_time
                        + (expected_ticks as f64 * config.interval_ms as f64)
                        + config.interval_ms as f64;
                }
            }

            let ticks = state.ticks_elapsed;
            drop(state);

            (on_tick_clone)(ticks);
            timer_clone.schedule_tick();
        });

        self.timeout_handle.set(Some(handle));
    }
}

impl<F> Drop for CountdownTimer<F>
where
    F: Fn(usize) + 'static,
{
    fn drop(&mut self) {
        self.stop();
    }
}
