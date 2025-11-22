pub mod countdown_timer;
pub mod emomtimer {
    pub enum Msg {
        Start,
        Stop,
        Reset,
        IncrementSecond,
        DecrementSecond,
        IncrementQuarter,
        DecrementQuarter,
        IncrementRound,
        DecrementRound,
        Tick,
    }

    pub const DEFAULT_MINUTES: usize = 1;
    pub const DEFAULT_SECONDS: usize = 0;
    pub const DEFAULT_ROUNDS: usize = 5;

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Time {
        pub seconds: usize,
        pub minutes: usize,
        pub tenths: usize,
    }

    impl Time {
        pub fn reset(&mut self) {
            self.seconds = DEFAULT_SECONDS;
            self.minutes = DEFAULT_MINUTES;
            self.tenths = 0;
        }

        pub fn is_zero(&self) -> bool {
            self.seconds == 0 && self.minutes == 0 && self.tenths == 0
        }

        pub fn tick(&mut self, max_seconds: usize) {
            if self.is_zero() {
                return;
            }
            // Decrement tenths first
            if self.tenths > 0 {
                self.tenths -= 1;
            } else {
                // tenths is 0, so we need to decrement seconds and wrap tenths to 9
                self.decrement_seconds(max_seconds);
                // Only set tenths to 9 if we didn't hit zero
                if !self.is_zero() {
                    self.tenths = 9;
                }
            }
        }

        pub fn increment_seconds(&mut self) {
            self.seconds += 1;
            if self.seconds == 60 {
                self.seconds = 0;
                self.increment_minutes()
            }
        }

        pub fn decrement_seconds(&mut self, max_seconds: usize) {
            if self.is_zero() {
                self.seconds = 0;
                self.minutes = 0;
                self.tenths = 0;
            } else if self.seconds == 0 {
                self.seconds = max_seconds - 1;
                self.decrement_minutes()
            } else {
                self.seconds -= 1;
            }
        }

        pub fn increment_quarter(&mut self) {
            self.seconds += 15;
            if self.seconds >= 60 {
                self.seconds -= 60;
                self.increment_minutes();
            }
        }

        pub fn decrement_quarter(&mut self) {
            if self.minutes == 0 && self.seconds < 15 {
                self.seconds = 0;
                self.minutes = 0;
                self.tenths = 0;
            } else if self.seconds < 15 {
                self.seconds += 45;
                self.decrement_minutes();
            } else {
                self.seconds -= 15;
            }
        }

        pub fn increment_minutes(&mut self) {
            self.minutes += 1;
        }

        pub fn decrement_minutes(&mut self) {
            if self.minutes > 0 {
                self.minutes -= 1;
            }
        }

        // ignoring tenths
        pub fn total_seconds(&self) -> usize {
            self.seconds + self.minutes * 60
        }
    }

    pub struct Timer {
        pub current_time: Time,
        pub rounds: usize,
        pub current_round: usize,
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

    pub fn distance<T>(a: T, b: T) -> T
    where
        T: std::ops::Sub<Output = T> + std::cmp::PartialOrd + Copy,
    {
        if a > b { a - b } else { b - a }
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
                    tenths: 1,
                },
                rounds: 10,
                current_round: 5,
                running: true,
            };
            timer.reset();
            assert_eq!(timer.current_time.seconds, DEFAULT_SECONDS);
            assert_eq!(timer.current_time.minutes, DEFAULT_MINUTES);
            assert_eq!(timer.current_time.tenths, 0);
            assert_eq!(timer.rounds, DEFAULT_ROUNDS);
            assert_eq!(timer.current_round, 1);
            assert!(!timer.running);
        }

        #[test]
        fn test_timer_increment_rounds() {
            let mut timer = Timer {
                current_time: Time {
                    seconds: 0,
                    minutes: 1,
                    tenths: 0,
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
                    tenths: 0,
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
                tenths: 0,
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
                tenths: 0,
            };
            time.decrement_seconds(60);
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
            time.seconds = 1;
            time.decrement_seconds(60);
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
            time.seconds = 0;
            time.minutes = 1;
            time.decrement_seconds(60);
            assert_eq!(time.seconds, 59);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
        }

        #[test]
        fn test_increment_quarter() {
            let mut time = Time {
                seconds: 0,
                minutes: 0,
                tenths: 0,
            };
            time.increment_quarter();
            assert_eq!(time.seconds, 15);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
            time.seconds = 45;
            time.increment_quarter();
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 1);
            assert_eq!(time.tenths, 0);
        }

        #[test]
        fn test_decrement_quarter() {
            let mut time = Time {
                seconds: 0,
                minutes: 0,
                tenths: 0,
            };
            time.decrement_quarter();
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
            time.seconds = 15;
            time.decrement_quarter();
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
            time.seconds = 0;
            time.minutes = 1;
            time.decrement_quarter();
            assert_eq!(time.seconds, 45);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
            time.seconds = 12;
            time.decrement_quarter();
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
        }

        #[test]
        fn test_time_reset() {
            let mut time = Time {
                seconds: 30,
                minutes: 2,
                tenths: 3,
            };
            time.reset();
            assert_eq!(time.seconds, DEFAULT_SECONDS);
            assert_eq!(time.minutes, DEFAULT_MINUTES);
            assert_eq!(time.tenths, 0);
        }

        #[test]
        fn test_time_increment_minutes() {
            let mut time = Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            };
            time.increment_minutes();
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 2);
            assert_eq!(time.tenths, 0);
        }

        #[test]
        fn test_time_decrement_minutes() {
            let mut time = Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            };
            time.decrement_minutes();
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
            time.decrement_minutes();
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
        }

        #[test]
        fn test_time_tick() {
            let mut time = Time {
                seconds: 0,
                minutes: 1,
                tenths: 0,
            };
            time.tick(60);
            assert_eq!(time.seconds, 59);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 9);
            time.tenths = 0;
            time.tick(60);
            assert_eq!(time.seconds, 58);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 9);
            time.seconds = 0;
            time.minutes = 0;
            time.tenths = 1;
            time.tick(60);
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
        }

        #[test]
        fn test_time_double_tick() {
            let mut time = Time {
                seconds: 0,
                minutes: 0,
                tenths: 0,
            };
            time.tick(60);
            assert_eq!(time.seconds, 0);
            assert_eq!(time.minutes, 0);
            assert_eq!(time.tenths, 0);
        }

        #[test]
        fn test_time_zero() {
            let mut time = Time {
                seconds: 0,
                minutes: 0,
                tenths: 0,
            };
            assert!(time.is_zero());
            time.seconds = 1;
            assert!(!time.is_zero());
            time.seconds = 0;
            time.minutes = 1;
            assert!(!time.is_zero());
            time.minutes = 0;
            time.tenths = 1;
            assert!(!time.is_zero());
        }

        #[test]
        fn test_distance() {
            assert_eq!(distance(1, 2), 1);
            assert_eq!(distance(2, 1), 1);
            assert_eq!(distance(1, 1), 0);
        }

        #[test]
        fn test_tick_countdown_sequence() {
            // Test that ticking properly counts down through all tenths
            // This test ensures we don't skip the second-to-last second
            let mut time = Time {
                seconds: 14,
                minutes: 0,
                tenths: 2,
            };
            // Tick from 14.2 -> 14.1
            time.tick(60);
            assert_eq!(time.seconds, 14);
            assert_eq!(time.tenths, 1);
            // Tick from 14.1 -> 14.0
            time.tick(60);
            assert_eq!(time.seconds, 14);
            assert_eq!(time.tenths, 0);
            // Tick from 14.0 -> 13.9 (this is where the bug was)
            time.tick(60);
            assert_eq!(time.seconds, 13);
            assert_eq!(time.tenths, 9);
            // Continue ticking
            time.tick(60);
            assert_eq!(time.seconds, 13);
            assert_eq!(time.tenths, 8);
        }

        #[test]
        fn test_15_second_timer_countdown() {
            // Test a 15 second timer counting down
            // Timer starts at 15.0 and should count: 15.0, 14.9, 14.8, ..., 14.1, 14.0, 13.9, ...
            let mut time = Time {
                seconds: 15,
                minutes: 0,
                tenths: 0,
            };

            // First tick from 15.0 should go to 14.9 (decrement second, set tenths to 9)
            time.tick(60);
            assert_eq!(time.seconds, 14, "After first tick from 15.0");
            assert_eq!(time.tenths, 9, "After first tick from 15.0");

            // Continue counting down second 14: 14.9 -> 14.8 -> ... -> 14.0
            for expected_tenth in (0..=8).rev() {
                time.tick(60);
                assert_eq!(
                    time.seconds, 14,
                    "Second mismatch at tenth {}",
                    expected_tenth
                );
                assert_eq!(
                    time.tenths, expected_tenth,
                    "Tenth mismatch at {}",
                    expected_tenth
                );
            }

            // Next tick from 14.0 should go to 13.9
            time.tick(60);
            assert_eq!(time.seconds, 13);
            assert_eq!(time.tenths, 9);
        }

        #[test]
        fn test_full_15_second_countdown() {
            // Simulate complete 15 second countdown to catch any skips
            let mut time = Time {
                seconds: 15,
                minutes: 0,
                tenths: 0,
            };

            let mut history = Vec::new();

            // Count down all the way to zero
            while !time.is_zero() {
                history.push((time.minutes, time.seconds, time.tenths));
                time.tick(60);
            }

            // Debug: print the history around 14
            println!("History around 14:");
            for (i, (m, s, t)) in history.iter().enumerate() {
                if *s >= 13 && *s <= 15 {
                    println!("  [{}]: {}:{}.{}", i, m, s, t);
                }
            }

            println!("Total history length: {}", history.len());

            // Should start at 15.0
            assert_eq!(history[0], (0, 15, 0));
            // First tick to 14.9
            assert_eq!(history[1], (0, 14, 9));
        }

        #[test]
        fn test_total_seconds() {
            let mut time = Time {
                seconds: 0,
                minutes: 0,
                tenths: 0,
            };
            assert_eq!(time.total_seconds(), 0);
            time.seconds = 1;
            assert_eq!(time.total_seconds(), 1);
            time.seconds = 0;
            time.minutes = 1;
            assert_eq!(time.total_seconds(), 60);
            time.seconds = 1;
            assert_eq!(time.total_seconds(), 61);
        }
    }
}
