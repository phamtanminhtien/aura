/// ABI bridging definitions between the compiled code and the Aura runtime.
///
/// This module defines exactly how the compiler generates stack maps
/// and expects the GC to behave regarding calling conventions.
use std::collections::HashSet;

/// A Stack Map describes which registers/stack slots contain active GC pointers
/// at a specific instruction offset (usually immediately after a `Call`).
///
/// During a minor or major garbage collection, the GC unwinds the stack
/// and uses these maps to find the precise set of live "roots".
#[derive(Debug, Clone)]
pub struct StackMap {
    /// The offset in bytes from the start of the function where this map applies.
    pub instruction_offset: usize,
    /// Bit vector or list of offset indices representing stack slots or registers
    /// currently holding live GC objects.
    pub gc_roots: HashSet<String>,
}

#[derive(Debug, Default)]
pub struct FunctionAbiInfo {
    pub stack_maps: Vec<StackMap>,
}
