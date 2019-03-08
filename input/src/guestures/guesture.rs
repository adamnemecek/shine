use crate::State;
use std::any::Any;

pub trait Guesture: Send + Sync {
    fn as_any(&self) -> &Any;

    fn on_update(&mut self, prev_state: &State, state: &mut State);
}
