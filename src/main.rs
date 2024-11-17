use gloo_timers::callback::Timeout;
use js_sys::Date;
use log::info;
use yew::{html, Component, Context, Html};

use emom::emomtimer::{Msg, Time, Timer, DEFAULT_MINUTES, DEFAULT_ROUNDS, DEFAULT_SECONDS};

const BLINKED_COUNT: usize = 4;

pub struct App {
    round_time: Time,
    timer: Timer,
    blinked: bool,
    timeout_handle: Option<Timeout>,
    next_tick_time: f64,
}

impl App {
    fn start(&mut self, ctx: &Context<Self>) {
        self.cancel(); // Cancel any existing timeout
        self.timer.current_round = 1;
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
        } else if self.timer.current_round > 1 && self.timer.current_time.tenths == 0 {
            self.toggle_blinked();
        }

        // Schedule the next tick
        self.schedule_tick(ctx);
    }

    fn tick_update_end_of_round(&mut self) {
        info!("end of round");
        self.timer.current_time = self.round_time;
        self.blinked = !self.blinked;

        if self.timer.current_round >= self.timer.rounds {
            info!("end of timer");
            self.cancel();
        } else {
            self.timer.current_round += 1;
        }
    }

    fn cancel(&mut self) {
        if let Some(handle) = self.timeout_handle.take() {
            handle.cancel();
        }
        self.timer.running = false;
        self.blinked = false;
    }

    fn reset(&mut self) {
        if let Some(handle) = self.timeout_handle.take() {
            handle.cancel();
        }
        self.round_time.reset();
        self.timer.reset();
        self.blinked = false;
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

    fn toggle_blinked_off(&mut self) {
        if emom::emomtimer::distance::<_>(
            self.round_time.total_seconds(),
            self.timer.current_time.total_seconds(),
        ) >= BLINKED_COUNT
            && self.blinked
        {
            self.blinked = false;
        }
    }

    fn toggle_blinked(&mut self) {
        if emom::emomtimer::distance::<_>(
            self.round_time.total_seconds(),
            self.timer.current_time.total_seconds(),
        ) < BLINKED_COUNT
        {
            self.blinked = !self.blinked;
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
            blinked: false,
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
                self.toggle_blinked_off();
                true
            }
            Msg::DecrementRound => {
                info!("decrementing rounds");
                self.timer.decrement_rounds();
                self.toggle_blinked_off();
                true
            }
            Msg::IncrementSecond => {
                info!("incrementing seconds");
                self.round_time.increment_seconds();
                self.timer.current_time.increment_seconds();
                self.toggle_blinked_off();
                true
            }
            Msg::DecrementSecond => {
                info!("decrementing seconds");
                let max_seconds = self.max_seconds();
                self.round_time.decrement_seconds(max_seconds);
                self.timer.current_time.decrement_seconds(max_seconds);
                self.toggle_blinked_off();
                true
            }
            Msg::IncrementQuarter => {
                info!("incrementing 15");
                self.round_time.increment_quarter();
                self.timer.current_time.increment_quarter();
                self.toggle_blinked_off();
                true
            }
            Msg::DecrementQuarter => {
                info!("decrementing 15");
                self.round_time.decrement_quarter();
                self.timer.current_time.decrement_quarter();
                self.toggle_blinked_off();
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
            <body style={if self.blinked { "color:red" } else { "color:black" }} >
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
            blinked: false,
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
            blinked: false,
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
            blinked: false,
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
            blinked: false,
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
            blinked: false,
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
            blinked: false,
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
            blinked: false,
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
            blinked: false,
            timeout_handle: None,
            next_tick_time: 0.0,
        };
        assert_eq!(app.max_seconds(), 60);
    }
}
