OS State Maestro
================

Deterministic, zero-dependency state machine and event orchestration for operating system development. Works in `no_std`, forbids `unsafe`, and offers predictable memory usage through fixed-size structures.

Features
--------
- Zero dependencies, `no_std` by default
- Explicit transitions via `Transition`
- Bounded `EventQueue<T, N>` with fixed capacity
- Minimal `StateMachine<S, E, H>` with compile-time generics
- Tick-based `Timeout` utilities without hardware assumptions
- Exhaustive docs and examples for docs.rs

Quick Example
-------------
```rust
extern crate std;
use os_state_maestro::{StateMachine, Transition, EventQueue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ev { Start, Stop }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum St { Init, Running, Halted }

fn handler(s: St, e: &Ev) -> Transition<St> {
    match (s, e) {
        (St::Init, Ev::Start) => Transition::to(St::Running),
        (St::Running, Ev::Stop) => Transition::to(St::Halted),
        _ => Transition::stay(),
    }
}

let mut sm = StateMachine::new(St::Init, handler);
let mut q: EventQueue<Ev, 4> = EventQueue::new();
q.push(Ev::Start).unwrap();
q.push(Ev::Stop).unwrap();
sm.run(&mut q);
assert_eq!(sm.state(), St::Halted);
```

License
-------
MIT License. Created by an AI Assistant, based on an idea by alisio85.

Contributing
------------
Development targets Rust edition 2024 and aims for warnings-free builds. Please run:
```
cargo clippy -- -D warnings
cargo test
```
