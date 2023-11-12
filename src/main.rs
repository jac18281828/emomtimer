use gloo_timers::callback::Interval;
use yew::{html, Component, Context, Html};

use emom::{Msg, Timer};

pub struct App {
    timer: Timer,
    interval: Option<Interval>,
}

impl App {
    fn cancel(&mut self) {
        self.interval = None;
    }
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            timer: Timer {
                seconds: 0,
                minutes: 1,
                rounds: 15,
                current_round: 1,
                running: false,
            },
            interval: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Start => {
                let link = ctx.link().clone();
                let tick_callback = move || link.send_message(Msg::Tick);
                let handle = Interval::new(1000, tick_callback);
                self.interval = Some(handle);
                true
            }
            Msg::Stop => {
                self.cancel();
                true
            }
            Msg::Tick => {
                self.timer.decrement_seconds();
                true
            }
            Msg::Reset => {
                self.timer.reset();
                true
            }
            Msg::IncrementRound => {
                self.timer.increment_rounds();
                true
            }
            Msg::DecrementRound => {
                self.timer.decrement_rounds();
                true
            }
            Msg::IncrementSecond => {
                self.timer.increment_seconds();
                true
            }
            Msg::DecrementSecond => {
                self.timer.decrement_seconds();
                true
            }
            Msg::IncrementMinute => {
                self.timer.increment_minutes();
                true
            }
            Msg::DecrementMinute => {
                self.timer.decrement_minutes();
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
            <body>
                <div class="mainTitle" align="right"><h1>{ "EMOM Timer" }</h1></div>
                <div class="timerDisplay" id="timerDisplay">{ format!("{}:{:02}", state.minutes, state.seconds) }</div>
                <div class="roundsDisplay" id="roundsDisplay">{ format!("{}/{}", state.current_round, state.rounds) }</div>
                <div id="buttonDisplay">
                <button onclick={ start } id="startButton">{ "Start" }</button>
                <button onclick={ stop } id="stopButton">{ "Stop" }</button>
                <button onclick={ reset } id="resetButton">{ "Reset" }</button>
                <button onclick={ on_add_round } id="incrementRoundButton">{ "+Round" }</button>
                <button onclick={ on_subtract_round } id="decrementRoundButton">{ "-Round" }</button>
                <button onclick={ on_add_minute } id="incrementMinuteButton">{ "+1:00" }</button>
                <button onclick={ on_subtract_minute } id="decrementMinuteButton">{ "-1:00" }</button>
                <button onclick={ on_add_second } id="incrementSecondButton">{ "+1" }</button>
                <button onclick={ on_subtract_second } id="decrementSecondButton">{ "-1" }</button>
                </div>
            </body>
            </html>
        }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
