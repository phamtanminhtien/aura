pub mod heap;
pub mod sweep;

pub use heap::{GcHeap, GcObject, GcTag};
pub use sweep::{RootSet, Sweeper};
