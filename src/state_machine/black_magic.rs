use enum_dispatch::enum_dispatch;
use std::fmt::Debug;

use crate::state_machine::{RocketEvents, RocketStates, RocketStateMachineContext};

/// Trait that all states implement. Ignore this, not super important
#[enum_dispatch]
pub trait State: Debug {
    fn enter(&self) {
        println!("Enter {:?}", self)
    }
    fn exit(&self) {
        println!("Exit {:?}", self)
    }
    fn event(&mut self, _event: RocketEvents) -> Option<RocketStates> {
        None
    }
    fn step(&mut self, context: &RocketStateMachineContext) -> Option<RocketStates>;

}


/// Transition Trait
pub trait TransitionInto<T> {
    fn transition(&self) -> T;
}

#[macro_export]
macro_rules! transition {
    ($self:ident, $i:ident) => {
        Some(TransitionInto::<$i>::transition($self).into())
    }
}

#[macro_export]
macro_rules! no_transition {
    () => {
        None
    }
}
