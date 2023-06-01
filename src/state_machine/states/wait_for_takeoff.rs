use crate::{no_transition, transition};
use crate::state_machine::{RocketStateMachineContext, RocketStates, State, TransitionInto};
use crate::state_machine::states::ascent::Ascent;
use crate::state_machine::states::initializing::Initializing;

#[derive(Debug)]
pub struct WaitForTakeoff {}

impl State for WaitForTakeoff {
    fn step(&mut self, context: &RocketStateMachineContext) -> Option<RocketStates> {
        if context.data.accel < 20.0 {
            no_transition!()
        } else {
            transition!(self, Ascent)
        }
    }
}

impl TransitionInto<WaitForTakeoff> for Initializing {
    fn transition(&self) -> WaitForTakeoff {
        WaitForTakeoff {}
    }
}
