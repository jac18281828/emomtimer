pub enum Msg {
    Start,
    Stop,
    Reset,
    IncrementSecond,
    DecrementSecond,
    IncrementMinute,
    DecrementMinute,
    IncrementRound,
    DecrementRound,
    Tick,
}

pub const DEFAULT_MINUTES: u32 = 1;
pub const DEFAULT_SECONDS: u32 = 0;
pub const DEFAULT_ROUNDS: u32 = 10;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Time {
    pub seconds: u32,
    pub minutes: u32,
}

impl Time {
    pub fn reset(&mut self) {
        self.seconds = DEFAULT_SECONDS;
        self.minutes = DEFAULT_MINUTES;
    }

    pub fn increment_seconds(&mut self) {
        self.seconds += 1;
        if self.seconds == 60 {
            self.seconds = 0;
            self.minutes += 1;
        }
    }

    pub fn decrement_seconds(&mut self) {
        if self.seconds == 0 && self.minutes == 0 {
            self.seconds = 0;
            self.minutes = 0;
        } else if self.seconds == 0 {
            self.seconds = 59;
            self.minutes -= 1;
        } else {
            self.seconds -= 1;
        }
    }

    pub fn increment_minutes(&mut self) {
        self.minutes += 1;
    }

    pub fn decrement_minutes(&mut self) {
        if self.minutes == 0 {
            self.minutes = 0;
        } else {
            self.minutes -= 1;
        }
    }
}

pub struct Timer {
    pub current_time: Time,
    pub rounds: u32,
    pub current_round: u32,
    pub running: bool,
}

impl Timer {
    pub fn reset(&mut self) {
        self.current_time.reset();
        self.rounds = DEFAULT_ROUNDS;
        self.current_round = 1;
        self.running = false;
    }

    pub fn increment_rounds(&mut self) {
        self.rounds += 1;
    }

    pub fn decrement_rounds(&mut self) {
        if self.rounds == 1 {
            self.rounds = 1;
        } else {
            self.rounds -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timer_reset() {
        let mut timer = Timer {
            current_time: Time {
                seconds: 30,
                minutes: 2,
            },
            rounds: 10,
            current_round: 5,
            running: true,
        };
        timer.reset();
        assert_eq!(timer.current_time.seconds, DEFAULT_SECONDS);
        assert_eq!(timer.current_time.minutes, DEFAULT_MINUTES);
        assert_eq!(timer.rounds, DEFAULT_ROUNDS);
        assert_eq!(timer.current_round, 1);
        assert_eq!(timer.running, false);
    }

    #[test]
    fn test_timer_increment_rounds() {
        let mut timer = Timer {
            current_time: Time {
                seconds: 0,
                minutes: 1,
            },
            rounds: 15,
            current_round: 1,
            running: true,
        };
        timer.increment_rounds();
        assert_eq!(timer.rounds, 16);
    }

    #[test]
    fn test_decrement_rounds() {
        let mut timer = Timer {
            current_time: Time {
                seconds: 0,
                minutes: 1,
            },
            rounds: 15,
            current_round: 1,
            running: true,
        };
        timer.decrement_rounds();
        assert_eq!(timer.rounds, 14);
        timer.rounds = 1;
        timer.decrement_rounds();
        assert_eq!(timer.rounds, 1);
    }

    #[test]
    fn test_time_increment_seconds() {
        let mut time = Time {
            seconds: 0,
            minutes: 1,
        };
        time.increment_seconds();
        assert_eq!(time.seconds, 1);
        assert_eq!(time.minutes, 1);
        time.seconds = 59;
        time.increment_seconds();
        assert_eq!(time.seconds, 0);
        assert_eq!(time.minutes, 2);
    }

    #[test]
    fn test_time_decrement_seconds() {
        let mut time = Time {
            seconds: 0,
            minutes: 0,
        };
        time.decrement_seconds();
        assert_eq!(time.seconds, 0);
        assert_eq!(time.minutes, 0);
        time.seconds = 1;
        time.decrement_seconds();
        assert_eq!(time.seconds, 0);
        assert_eq!(time.minutes, 0);
        time.seconds = 0;
        time.minutes = 1;
        time.decrement_seconds();
        assert_eq!(time.seconds, 59);
        assert_eq!(time.minutes, 0);
    }

    #[test]
    fn test_time_reset() {
        let mut time = Time {
            seconds: 30,
            minutes: 2,
        };
        time.reset();
        assert_eq!(time.seconds, DEFAULT_SECONDS);
        assert_eq!(time.minutes, DEFAULT_MINUTES);
    }
}
