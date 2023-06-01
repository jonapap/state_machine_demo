mod black_magic;
mod states;

use std::fmt::Debug;
use enum_dispatch::enum_dispatch;
use crate::{GPIOController, RocketData};
use crate::state_machine::states::*;

pub use black_magic::*;

// The actual state machine that encapsulates a state
pub struct RocketStateMachine {
    state: RocketStates,
}

// A context that is passed around when executing the state machine
pub struct RocketStateMachineContext<'a> {
    pub(crate) data: &'a RocketData,
    pub(crate) gpio: &'a GPIOController
}

// Define some functions to interact with the state machine
impl RocketStateMachine {
    pub fn new() -> Self {
        let state = Initializing {};
        state.enter();

        RocketStateMachine {
            state: state.into(),
        }
    }

    pub fn run(&mut self, data: &RocketStateMachineContext) {
        if let Some(new_state) = self.state.step(data) {
            self.state.exit();
            new_state.enter();
            self.state = new_state;
        }
    }
}

// All events are found here
pub enum RocketEvents {
    DeployDrogue,
    DeployMain,
}

// All states are defined here. Another struct must be defined for the actual state, and that struct
// must implement the State trait
#[enum_dispatch(State)]
#[derive(Debug)]
pub enum RocketStates {
    Initializing,
    WaitForTakeoff,
    Ascent,
    Apogee,
    Landed,
    Abort
}


