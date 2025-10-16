use alloc::vec::Vec;
use crate::thread::Thread;

pub struct Scheduler {
    tasks: Vec<Thread>
}