use crate::state_machine::{RocketStateMachineContext, RocketStates, State, TransitionInto};
use crate::state_machine::states::wait_for_takeoff::WaitForTakeoff;
use crate::transition;

#[derive(Debug)]
pub struct Initializing {}

impl State for Initializing {
    fn step(&mut self, data: &RocketStateMachineContext) -> Option<RocketStates> {
        // Check that all systems are ok, then transition
        transition!(self, WaitForTakeoff)
    }
}
