use crate::state_machine::{RocketStateMachineContext, RocketStates, State};

#[derive(Debug)]
pub struct Apogee {}

impl State for Apogee {
    fn step(&mut self, data: &RocketStateMachineContext) -> Option<RocketStates> {
        todo!()
    }
}
