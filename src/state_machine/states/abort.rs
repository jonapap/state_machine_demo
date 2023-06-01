use crate::state_machine::{RocketStateMachineContext, RocketStates, State, TransitionInto};
use crate::state_machine::states::initializing::Initializing;

#[derive(Debug)]
pub struct Abort {}

impl State for Abort {
    fn step(&mut self, data: &RocketStateMachineContext) -> Option<RocketStates> {
        todo!()
    }
}

impl TransitionInto<Abort> for Initializing {
    fn transition(&self) -> Abort {
        Abort {}
    }
}
