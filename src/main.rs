use gloo_timers::callback::Timeout;
use js_sys::Date;
use log::info;
use yew::{Component, Context, Html, html};

use emom::emomtimer::{DEFAULT_MINUTES, DEFAULT_ROUNDS, DEFAULT_SECONDS, Msg, Time, Timer};

const BLINKED_COUNT: usize = 3;

#[derive(Debug, Clone, Copy, PartialEq)]
enum BlinkState {
    Red,
    Green,
    None,
}

pub struct App {
    round_time: Time,
    timer: Timer,
    blink_state: BlinkState,
    timeout_handle: Option<Timeout>,
    next_tick_time: f64,
}

impl App {
    fn start(&mut self, ctx: &Context<Self>) {
        self.cancel(); // Cancel any existing timeout
        if self.timer.current_round >= self.timer.rounds {
            self.timer.current_round = 1;
        }
        let start_time = Date::now();
        self.next_tick_time = start_time + 100.0; // Schedule first tick after 100ms
        self.timer.running = true;
        self.schedule_tick(ctx);
    }

    fn schedule_tick(&mut self, ctx: &Context<Self>) {
        let now = Date::now();
        // extra millisecond to ensure tick is called prior to next tenth
        let delay = self.next_tick_time - now - 1.0;
        let delay = delay.max(0.0).floor() as u32;

        let link = ctx.link().clone();
        if self.timer.running {
            let handle = Timeout::new(delay, move || {
                link.send_message(Msg::Tick);
            });
            self.timeout_handle = Some(handle);
        } else {
            self.timeout_handle = None;
        }
    }

    fn tick(&mut self, ctx: &Context<Self>) {
        // Update the next tick time
        self.next_tick_time += 100.0; // Schedule next tick after 100ms

        // Handle the tick
        self.timer.current_time.tick(self.max_seconds());
        if self.timer.current_time.is_zero() {
            self.tick_update_end_of_round();
        } else {
            self.update_blink_state();
        }

        // Schedule the next tick
        self.schedule_tick(ctx);
    }

    fn tick_update_end_of_round(&mut self) {
        info!("end of round");
        self.timer.current_time = self.round_time;

        if self.timer.current_round >= self.timer.rounds {
            info!("end of timer");
            self.blink_state = BlinkState::None;
            self.cancel();
        } else {
            self.timer.current_round += 1;
            self.blink_state = BlinkState::None;
        }
    }

    fn cancel(&mut self) {
        if let Some(handle) = self.timeout_handle.take() {
            handle.cancel();
        }
        self.timer.running = false;
        self.blink_state = BlinkState::None;
    }

    fn reset(&mut self) {
        if let Some(handle) = self.timeout_handle.take() {
            handle.cancel();
        }
        self.round_time.reset();
        self.timer.reset();
        self.blink_state = BlinkState::None;
    }

    fn stop(&mut self) {
        info!("stopping");
        self.cancel();
    }

    fn max_seconds(&self) -> usize {
        if self.round_time.minutes > 0 {
            60
        } else {
            self.round_time.seconds.max(1)
        }
    }

    fn clear_blink_state(&mut self) {
        self.blink_state = BlinkState::None;
    }

    fn update_blink_state(&mut self) {
        let total_seconds = self.timer.current_time.total_seconds();
        let round_seconds = self.round_time.total_seconds();
        let tenths = self.timer.current_time.tenths;

        // Blink green at 1, 2, and 3 second marks after round starts
        // Only after round 1 has started
        // E.g., for 60 seconds: blink at 59, 58, 57 (when 1, 2, 3 seconds have elapsed)
        // Blink for 0.5 seconds (tenths 0-4) at the start of each second
        if self.timer.current_round > 1
            && total_seconds > round_seconds - (BLINKED_COUNT + 1)
            && total_seconds < round_seconds
            && tenths <= 4
        {
            self.blink_state = BlinkState::Green;
        }
        // Blink red at the end of the minute (last 3 seconds)
        // Blink for 0.5 seconds (tenths 0-4) at seconds 3, 2, 1
        else if total_seconds > 0 && total_seconds <= BLINKED_COUNT && tenths <= 4 {
            self.blink_state = BlinkState::Red;
        } else {
            self.blink_state = BlinkState::None;
        }
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        let time = Time {
            seconds: DEFAULT_SECONDS,
            minutes: DEFAULT_MINUTES,
            tenths: 0,
        };

        Self {
            round_time: time,
            timer: Timer {
                current_time: time,
                rounds: DEFAULT_ROUNDS,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                if self.timeout_handle.is_none() {
                    self.start(ctx);
                }
                true
            }
            Msg::Stop => {
                self.stop();
                true
            }
            Msg::Tick => {
                self.tick(ctx);
                true
            }
            Msg::Reset => {
                self.reset();
                true
            }
            Msg::IncrementRound => {
                info!("incrementing rounds");
                self.timer.increment_rounds();
                self.clear_blink_state();
                true
            }
            Msg::DecrementRound => {
                info!("decrementing rounds");
                self.timer.decrement_rounds();
                self.clear_blink_state();
                true
            }
            Msg::IncrementSecond => {
                info!("incrementing seconds");
                self.round_time.increment_seconds();
                self.timer.current_time.increment_seconds();
                self.clear_blink_state();
                true
            }
            Msg::DecrementSecond => {
                info!("decrementing seconds");
                let max_seconds = self.max_seconds();
                self.round_time.decrement_seconds(max_seconds);
                self.timer.current_time.decrement_seconds(max_seconds);
                self.clear_blink_state();
                true
            }
            Msg::IncrementQuarter => {
                info!("incrementing 15");
                self.round_time.increment_quarter();
                self.timer.current_time.increment_quarter();
                self.clear_blink_state();
                true
            }
            Msg::DecrementQuarter => {
                info!("decrementing 15");
                self.round_time.decrement_quarter();
                self.timer.current_time.decrement_quarter();
                self.clear_blink_state();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let state = &self.timer;
        let start = ctx.link().callback(|_| Msg::Start);
        let stop = ctx.link().callback(|_| Msg::Stop);
        let reset = ctx.link().callback(|_| Msg::Reset);
        let on_add_round = ctx.link().callback(|_| Msg::IncrementRound);
        let on_subtract_round = ctx.link().callback(|_| Msg::DecrementRound);
        let on_add_second = ctx.link().callback(|_| Msg::IncrementSecond);
        let on_subtract_second = ctx.link().callback(|_| Msg::DecrementSecond);
        let on_add_quarter = ctx.link().callback(|_| Msg::IncrementQuarter);
        let on_subtract_quarter = ctx.link().callback(|_| Msg::DecrementQuarter);

        html! {
            <html lang="en">
            <head>
                <meta charset="UTF-8" />
                <meta http-equiv="X-UA-Compatible" content="IE=edge" />
                <meta name="viewport" content="width=device-width, initial-scale=1.0" />
                <title>{ "EMOM Timer" }</title>
            </head>
            <body style={
                match self.blink_state {
                    BlinkState::Red => "color:red",
                    BlinkState::Green => "color:green",
                    BlinkState::None => "color:black",
                }
            } >
                <div id="background">
                    <div class="mainTitle">
                        <h3>{ "EMOM Timer" }</h3>
                    </div>
                <div class="roundsDisplay" id="roundsDisplay">
                    <span>{ format!("{}/{}", state.current_round, state.rounds) }</span>
                    <span class="roundTime">{ format!("{}:{:02}", self.round_time.minutes, self.round_time.seconds) }</span>
                </div>
                <div class="timerDisplay" id="timerDisplay">
                    { format!("{}:{:02}.{}", state.current_time.minutes, state.current_time.seconds, state.current_time.tenths) }
                </div>
                <div id="buttonDisplay">
                    <button aria-label="Start" onclick={ start } id="startButton">{ "Start" }</button>
                    <button aria-label="Start" onclick={ stop } id="stopButton">{ "Pause" }</button>
                    <button aria-label="Reset" onclick={ reset } id="resetButton">{ "Reset" }</button>
                    <button aria-label="Decrement Round" onclick={ on_subtract_round } id="decrementRoundButton">{ "-Round" }</button>
                    <button aria-label="Increment Round" onclick={ on_add_round } id="incrementRoundButton">{ "+Round" }</button>
                    <button aria-label="Decrement 15" onclick={ on_subtract_quarter } id="decrementQuarterButton">{ "-15" }</button>
                    <button aria-label="Increment 15" onclick={ on_add_quarter } id="incrementQuarterButton">{ "+15" }</button>
                    <button aria-label="Decrement Second" onclick={ on_subtract_second } id="decrementSecondButton">{ "-1" }</button>
                    <button aria-label="Increment Second" onclick={ on_add_second } id="incrementSecondButton">{ "+1" }</button>
                </div>
                <h5><a href="https://github.com/jac18281828/emomtimer">{ "GitHub" }</a></h5>
                </div>
            </body>
            </html>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    info!("Starting up");
    yew::Renderer::<App>::new().render();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_seconds() {
        let app = App {
            round_time: Time {
                seconds: 30,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 30,
                    minutes: 1,
                    tenths: 0,
                },
                rounds: 1,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 60);
    }

    #[test]
    fn test_max_seconds_zero() {
        let app = App {
            round_time: Time {
                seconds: 0,
                minutes: 0,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 0,
                    minutes: 0,
                    tenths: 0,
                },
                rounds: 1,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 1);
    }

    #[test]
    fn test_max_seconds_one() {
        let app = App {
            round_time: Time {
                seconds: 1,
                minutes: 0,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 1,
                    minutes: 0,
                    tenths: 0,
                },
                rounds: 1,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 1);
    }

    #[test]
    fn test_max_seconds_one_minute() {
        let app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 0,
                    minutes: 1,
                    tenths: 0,
                },
                rounds: 1,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 60);
    }

    #[test]
    fn test_max_seconds_one_minute_one_second() {
        let app = App {
            round_time: Time {
                seconds: 1,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 1,
                    minutes: 1,
                    tenths: 0,
                },
                rounds: 1,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 60);
    }

    #[test]
    fn test_max_seconds_one_minute_one_second_one_tenth() {
        let app = App {
            round_time: Time {
                seconds: 1,
                minutes: 1,
                tenths: 1,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 1,
                    minutes: 1,
                    tenths: 1,
                },
                rounds: 1,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 60);
    }

    #[test]
    fn test_max_seconds_one_minute_one_tenth() {
        let app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 1,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 0,
                    minutes: 1,
                    tenths: 1,
                },
                rounds: 1,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 60);
    }

    #[test]
    fn test_max_seconds_one_minute_one_tenth_zero_seconds() {
        let app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 1,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 0,
                    minutes: 1,
                    tenths: 0,
                },
                rounds: 1,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 60);
    }

    #[test]
    fn test_red_blink_at_3_seconds() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 3,
                    minutes: 0,
                    tenths: 0,
                },
                rounds: 10,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::Red);
    }

    #[test]
    fn test_red_blink_at_3_seconds_middle() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 3,
                    minutes: 0,
                    tenths: 4,
                },
                rounds: 10,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::Red);
    }

    #[test]
    fn test_no_red_blink_at_3_seconds_late() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 3,
                    minutes: 0,
                    tenths: 5,
                },
                rounds: 10,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::None);
    }

    #[test]
    fn test_red_blink_at_1_second() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 1,
                    minutes: 0,
                    tenths: 2,
                },
                rounds: 10,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::Red);
    }

    #[test]
    fn test_no_red_blink_at_0_seconds() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 0,
                    minutes: 0,
                    tenths: 5,
                },
                rounds: 10,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::None);
    }

    #[test]
    fn test_no_green_blink_round_1() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 59,
                    minutes: 0,
                    tenths: 0,
                },
                rounds: 10,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::None);
    }

    #[test]
    fn test_green_blink_at_59_seconds_round_2() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 59,
                    minutes: 0,
                    tenths: 0,
                },
                rounds: 10,
                current_round: 2,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::Green);
    }

    #[test]
    fn test_green_blink_at_58_seconds_round_5() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 58,
                    minutes: 0,
                    tenths: 3,
                },
                rounds: 10,
                current_round: 5,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::Green);
    }

    #[test]
    fn test_no_green_blink_at_60_seconds() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 0,
                    minutes: 1,
                    tenths: 0,
                },
                rounds: 10,
                current_round: 2,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::None);
    }

    #[test]
    fn test_no_green_blink_at_56_seconds() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 56,
                    minutes: 0,
                    tenths: 0,
                },
                rounds: 10,
                current_round: 2,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::None);
    }

    #[test]
    fn test_clear_blink_state() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 3,
                    minutes: 0,
                    tenths: 0,
                },
                rounds: 10,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::Red,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.clear_blink_state();
        assert_eq!(app.blink_state, BlinkState::None);
    }

    #[test]
    fn test_no_blink_at_30_seconds() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 30,
                    minutes: 0,
                    tenths: 5,
                },
                rounds: 10,
                current_round: 3,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::None);
    }

    #[test]
    fn test_green_blink_at_57_seconds() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 57,
                    minutes: 0,
                    tenths: 4,
                },
                rounds: 10,
                current_round: 2,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::Green);
    }

    #[test]
    fn test_red_blink_at_2_seconds() {
        let mut app = App {
            round_time: Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            },
            timer: Timer {
                current_time: Time {
                    seconds: 2,
                    minutes: 0,
                    tenths: 1,
                },
                rounds: 10,
                current_round: 1,
                running: false,
            },
            blink_state: BlinkState::None,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        app.update_blink_state();
        assert_eq!(app.blink_state, BlinkState::Red);
    }
}
