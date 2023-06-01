use crate::state_machine::{RocketStateMachineContext, RocketStates, State};

#[derive(Debug)]
pub struct Landed {}

impl State for Landed {
    fn step(&mut self, data: &RocketStateMachineContext) -> Option<RocketStates> {
        todo!()
    }
}
