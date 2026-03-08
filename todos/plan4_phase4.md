# Plan 4 Phase 4: Compiler-Runtime Contract

## 📌 Goals

Define how the compiler should generate code that interacts with the GC. This serves as the ABI bridge between the generated native Code and the `.rs` runtime we've written (GC, Scheduler). Specifically, we need to design stack maps for root tracing and write barriers for generational GC.

## 📝 Tasks

- [x] Create `src/compiler/backend/abi.rs` to define the standard ABI contract (register reservations, calling conventions mapping to the runtime).
- [x] Add `WriteBarrier` instruction to the SSA IR in `src/compiler/ir/instr.rs`.
- [x] Update IR `Lowerer` (`src/compiler/ir/lower.rs`) to emit `WriteBarrier` instructions when modifying GC-managed object fields.
- [x] Define the Stack Map structure in `src/compiler/backend/abi.rs` for tracking GC roots across function calls.
- [x] Write unit tests to ensure that the IR lowers correctly with write barriers in place.
