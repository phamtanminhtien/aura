# Plan 4 Phase 1: Generational GC

## 📌 Goals

Implement a basic generational garbage collector in Rust inside `src/runtime/gc/`. This is the foundation for Aura's memory model and allows heap-allocated objects to be safely reclaimed.

## 📝 Tasks

- [/] Create `src/runtime/gc/heap.rs` — define `GcHeap` with young/old generation arenas and `alloc()` API
- [ ] Create `src/runtime/gc/sweep.rs` — implement `minor_gc()` (scavenge) and `major_gc()` (mark-and-sweep)
- [ ] Create `src/runtime/gc/mod.rs` — expose `GcHeap` and GC trigger logic
- [ ] Create `src/runtime/mod.rs` — expose `gc` submodule from runtime
- [ ] Add `GcObject` header struct (type tag + forward pointer for copying GC)
- [ ] Write unit tests for alloc + minor GC collection cycle
- [ ] Verify with a stress test: allocate many objects and confirm no leaks/panics
