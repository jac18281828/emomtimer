use gloo_timers::callback::Interval;
use log::{info, warn};
use yew::{html, Component, Context, Html};

use emom::emomtimer::{Msg, Time, Timer, DEFAULT_MINUTES, DEFAULT_ROUNDS, DEFAULT_SECONDS};

pub struct App {
    round_time: Time,
    timer: Timer,
    blinked: bool,
    interval: Option<Interval>,
}

impl App {
    fn cancel(&mut self) {
        if let Some(intr_val) = self.interval.take() {
            intr_val.cancel();
        }
        self.timer.running = false;
        self.blinked = false;
    }

    fn reset(&mut self) {
        self.round_time.reset();
        self.timer.reset();
        self.blinked = false;
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
            interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                if self.interval.is_none() {
                    info!("starting");
                    let link = ctx.link().clone();
                    let tick_callback = move || link.send_message(Msg::Tick);
                    let handle = Interval::new(98, tick_callback);
                    self.interval = Some(handle);
                }
                true
            }
            Msg::Stop => {
                info!("stopping");
                self.cancel();
                true
            }
            Msg::Tick => {
                self.timer.current_time.tick();
                let max_seconds = if self.round_time.seconds == 0 {
                    60
                } else {
                    self.round_time.seconds
                };
                if self.timer.current_time.is_zero() {
                    info!("end of round");
                    self.timer.current_round += 1;
                    self.timer.current_time = self.round_time;
                    self.blinked = !self.blinked;

                    if self.timer.current_round > self.timer.rounds {
                        info!("end of timer");
                        self.timer.current_round = 1;
                        self.cancel();
                    }
                } else if self.timer.current_round > 1
                    && max_seconds - self.timer.current_time.seconds < 4
                    && self.timer.current_time.tenths == 0
                {
                    self.blinked = !self.blinked;
                }
                true
            }
            Msg::Reset => {
                warn!("resetting");
                self.reset();
                true
            }
            Msg::IncrementRound => {
                info!("incrementing rounds");
                self.timer.increment_rounds();
                true
            }
            Msg::DecrementRound => {
                info!("decrementing rounds");
                self.timer.decrement_rounds();
                true
            }
            Msg::IncrementSecond => {
                info!("incrementing seconds");
                self.round_time.increment_seconds();
                self.timer.current_time = self.round_time;
                true
            }
            Msg::DecrementSecond => {
                info!("decrementing seconds");
                self.round_time.decrement_seconds();
                self.timer.current_time = self.round_time;
                true
            }
            Msg::IncrementMinute => {
                info!("incrementing minutes");
                self.round_time.increment_minutes();
                self.timer.current_time = self.round_time;
                true
            }
            Msg::DecrementMinute => {
                info!("decrementing minutes");
                self.round_time.decrement_minutes();
                self.timer.current_time = self.round_time;
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
        let on_add_minute = ctx.link().callback(|_| Msg::IncrementMinute);
        let on_subtract_minute = ctx.link().callback(|_| Msg::DecrementMinute);

        html! {
            <html lang="en">
            <head>
            <meta charset="UTF-8" />
            <meta http-equiv="X-UA-Compatible" content="IE=edge" />
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <title>{ "EMOM Timer" }</title>
            </head>
            <body style={if self.blinked { "color:red" } else { "color:black" }} >
                <div class="mainTitle" align="right"><h1>{ "EMOM Timer" }</h1></div>
                <div class="roundsDisplay" id="roundsDisplay">{ format!("{}/{}", state.current_round, state.rounds) }</div>
                <div class="timerDisplay" id="timerDisplay">{ format!("{}:{:02}.{}", state.current_time.minutes, state.current_time.seconds, state.current_time.tenths) }</div>
                <div id="buttonDisplay">
                <button aria-label="Start" onclick={ start } id="startButton">{ "Start" }</button>
                <button aria-label="Stop" onclick={ stop } id="stopButton">{ "Stop" }</button>
                <button aria-label="Reset" onclick={ reset } id="resetButton">{ "Reset" }</button>
                <button aria-label="Increment Round" onclick={ on_add_round } id="incrementRoundButton">{ "+Round" }</button>
                <button aria-label="Decrement Round" onclick={ on_subtract_round } id="decrementRoundButton">{ "-Round" }</button>
                <button aria-label="Increment Minute" onclick={ on_add_minute } id="incrementMinuteButton">{ "+1:00" }</button>
                <button aria-label="Decrement Minute" onclick={ on_subtract_minute } id="decrementMinuteButton">{ "-1:00" }</button>
                <button aria-label="Increment Second" onclick={ on_add_second } id="incrementSecondButton">{ "+1" }</button>
                <button aria-label="Decrement Second" onclick={ on_subtract_second } id="decrementSecondButton">{ "-1" }</button>
                </div>
                <h5><a href="https://github.com/jac18281828/emomtimer">{ "GitHub" }</a></h5>
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
