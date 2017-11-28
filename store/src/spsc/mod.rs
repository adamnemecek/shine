use std::sync::Arc;

pub use self::spscstate::{Sender, Receiver};
pub use self::spscstate::{RefReceiveBuffer, RefSendBuffer};
use self::spscstate::TripleBuffer;

mod spscstate;

/// Create a Sender/Receiver with an embedded shared buffer for communication.
/// It is not a "Single Producer Single Consumer" queue as some massages can be dropped based
/// on thread scheduling.
pub fn state_channel<T: Default>() -> (Sender<T>, Receiver<T>) {
    let a = Arc::new(TripleBuffer::new());
    (Sender::new(&a), Receiver::new(&a))
}
