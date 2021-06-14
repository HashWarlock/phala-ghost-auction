#![no_std]

extern crate alloc;

mod dispatcher;
mod send_queue;
mod simple_mpsc;
mod types;

pub use dispatcher::MessageDispatcher;
pub use send_queue::{MessageSendHandle, MessageSendQueue, Signer};
pub use types::*;

// TODO.kevin: use std::sync::Mutex instead.
// See:
//    https://matklad.github.io/2020/01/02/spinlocks-considered-harmful.html
//    https://matklad.github.io/2020/01/04/mutexes-are-faster-than-spinlocks.html
use spin::mutex::Mutex;
