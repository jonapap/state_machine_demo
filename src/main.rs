use std::time::{Duration, Instant, SystemTime};
use crate::state_machine::{RocketStateMachine, RocketStateMachineContext};

mod state_machine;

struct GPIOController {}

impl GPIOController {
    pub fn set_high(&self, pin: u8) {

    }
}

struct RocketData {
    accel: f32
}

fn main() {
    let mut sm = RocketStateMachine::new();

    let mut data = RocketData {
        accel: 0.0,
    };

    let gpio = GPIOController {};

    let time = Instant::now();

    loop {
        if (Instant::now() - time) > Duration::from_secs(10) {
            data.accel = 100.0;
        }

        let context = RocketStateMachineContext {
            data: &data,
            gpio: &gpio
        };

        sm.run(&context)
    }
}
