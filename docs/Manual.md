OS State Maestro Manual
=======================

Overview
--------
OS State Maestro provides deterministic state management primitives for OS components: drivers, boot phases, schedulers, and services. It uses fixed-size data structures, requires no allocations, and forbids unsafe.

Core Concepts
-------------
- State: an application-defined type (usually an enum) representing discrete phases.
- Event: a data type that triggers state transitions.
- Transition: explicit outcome (Stay or To(next)).
- StateMachine: orchestrates transitions via a user-provided handler.
- EventQueue: bounded, single-threaded FIFO for events.
- TickCounter & Timeout: deterministic time helpers without hardware dependencies.

Design Principles
-----------------
- Determinism: all memory usage is compile-time bounded.
- Simplicity: minimal API surface that composes well.
- Purity: state transitions are calculated from `(state, event)`.
- Portability: `no_std` compatibility makes it suitable for kernels and embedded.
- Safety: no `unsafe`; behavior is well-defined and testable.

Usage Patterns
--------------
1. Define states and events as enums.
2. Write a handler `fn(State, &Event) -> Transition<State>`.
3. Instantiate `StateMachine::new(initial, handler)`.
4. Push events into `EventQueue<T, N>`.
5. Call `sm.run(&mut queue)` in your main loop.

Timeouts
--------
Use `TickCounter` to advance time (e.g., every timer interrupt) and `Timeout` to check elapsed conditions. This avoids direct hardware coupling.

Integration Notes
-----------------
- Interrupt Context: keep event pushing minimal; defer heavy work to the main loop.
- Memory Footprint: choose `N` for queue capacity carefully per subsystem needs.
- Testing: model state machines with exhaustive tests to ensure expected transitions.
- Logging/Tracing: instrument handlers to record transitions for diagnostics.

Examples
--------
See code examples embedded in the library documentation and README for canonical patterns.

License and Attribution
-----------------------
MIT License. Created by an AI Assistant, based on an idea by alisio85. See LICENSE file for details.
