#![no_std]
#![forbid(unsafe_code)]
#![deny(warnings)]
#![cfg_attr(docsrs, feature(doc_cfg))]
/*!
OS State Maestro
================

A zero-dependency, `no_std` state machine and event orchestration toolkit tailored for operating system development. It helps you design deterministic subsystems (drivers handshakes, boot phases, power states, task lifecycles) with compile-time sized structures, predictable memory footprint, and explicit transitions.

Design Goals
------------
- Deterministic: fixed-size structures with no allocations.
- Zero dependencies: only `core` and optional `std` for tests/examples.
- No unsafe: all functionality implemented without unsafe code.
- Portable: suitable for kernels, bootloaders, hypervisors, and embedded OSes.
- Documented: exhaustive API docs and examples for docs.rs.

Quick Start
-----------
```rust
extern crate std;
use os_state_maestro::{StateMachine, Transition, EventQueue};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ev { Start, Ready, Stop }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum MyState { Init, Running, Halted }

fn handler(state: MyState, ev: &Ev) -> Transition<MyState> {
    match (state, ev) {
        (MyState::Init, Ev::Start) => Transition::to(MyState::Running),
        (MyState::Running, Ev::Stop) => Transition::to(MyState::Halted),
        _ => Transition::stay(),
    }
}

let mut sm = StateMachine::new(MyState::Init, handler);
let mut q: EventQueue<Ev, 4> = EventQueue::new();
q.push(Ev::Start).unwrap();
q.push(Ev::Stop).unwrap();

sm.run(&mut q);
assert_eq!(sm.state(), MyState::Halted);
```

*/

/// A transition outcome produced by a state handler.
///
/// Transitions are explicit and immutable. They either stay in the current
/// state or move to a new state. No side-effects are performed by this type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Transition<S: Copy + Eq> {
    /// Remain in the current state.
    Stay,
    /// Move to the provided next state.
    To(S),
}

impl<S: Copy + Eq> Transition<S> {
    /// Create a transition that stays in the current state.
    #[inline]
    pub const fn stay() -> Self {
        Transition::Stay
    }
    /// Create a transition to the provided state.
    #[inline]
    pub const fn to(next: S) -> Self {
        Transition::To(next)
    }
}

/// A deterministic, zero-allocation event queue with fixed capacity.
///
/// This queue is single-threaded and intended for kernel/scheduler contexts
/// where cooperative processing is used. It provides predictable memory usage.
pub struct EventQueue<T, const N: usize> {
    buf: [Option<T>; N],
    head: usize,
    tail: usize,
    len: usize,
}

impl<T, const N: usize> EventQueue<T, N> {
    /// Create a new empty queue.
    #[inline]
    pub fn new() -> Self {
        Self {
            buf: core::array::from_fn(|_| None),
            head: 0,
            tail: 0,
            len: 0,
        }
    }
    /// Current number of elements in the queue.
    #[inline]
    pub const fn len(&self) -> usize {
        self.len
    }
    /// Queue capacity.
    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }
    /// Whether the queue is empty.
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// Whether the queue is full.
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.len == N
    }
    /// Push an element to the tail.
    ///
    /// Returns `Err(value)` if the queue is full.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            return Err(value);
        }
        self.buf[self.tail] = Some(value);
        self.tail = (self.tail + 1) % N;
        self.len += 1;
        Ok(())
    }
    /// Pop an element from the head.
    ///
    /// Returns `None` if empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let v = self.buf[self.head].take();
        self.head = (self.head + 1) % N;
        self.len -= 1;
        v
    }
    /// Clear the queue, dropping all elements.
    pub fn clear(&mut self) {
        while self.pop().is_some() {}
    }
}

impl<T, const N: usize> Drop for EventQueue<T, N> {
    fn drop(&mut self) {
        // Ensure all initialized elements are dropped on teardown.
        while self.pop().is_some() {}
    }
}

/// A minimal deterministic state machine.
///
/// - `S` is the user-defined state type (typically an enum).
/// - `E` is the event type processed by the machine.
/// - `H` is a handler: `fn(S, &E) -> Transition<S>`.
pub struct StateMachine<S: Copy + Eq, E, H: Fn(S, &E) -> Transition<S>> {
    state: S,
    handler: H,
    _event: core::marker::PhantomData<E>,
}

impl<S: Copy + Eq, E, H: Fn(S, &E) -> Transition<S>> StateMachine<S, E, H> {
    /// Create a new machine with an initial state and a handler.
    #[inline]
    pub const fn new(initial: S, handler: H) -> Self {
        Self {
            state: initial,
            handler,
            _event: core::marker::PhantomData,
        }
    }
    /// Current state.
    #[inline]
    pub const fn state(&self) -> S {
        self.state
    }
    /// Process a single event. Returns `true` if a transition occurred.
    pub fn step(&mut self, ev: &E) -> bool {
        match (self.handler)(self.state, ev) {
            Transition::Stay => false,
            Transition::To(next) => {
                self.state = next;
                true
            }
        }
    }
    /// Drain and process all events from a queue.
    pub fn run<const N: usize>(&mut self, q: &mut EventQueue<E, N>) {
        while let Some(ev) = q.pop() {
            let _ = self.step(&ev);
        }
    }
}

/// A simple tick counter for deterministic scheduling.
///
/// This type does not interact with hardware timers; it is advanced by the user
/// and may be used to implement timeouts in state handlers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TickCounter {
    ticks: u64,
}

impl TickCounter {
    /// Create a new counter starting at zero.
    #[inline]
    pub const fn new() -> Self {
        Self { ticks: 0 }
    }
    /// Advance by one tick.
    #[inline]
    pub fn tick(&mut self) {
        self.ticks = self.ticks.wrapping_add(1);
    }
    /// Current tick value.
    #[inline]
    pub const fn value(&self) -> u64 {
        self.ticks
    }
}

impl<T, const N: usize> Default for EventQueue<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for TickCounter {
    fn default() -> Self {
        Self::new()
    }
}
/// A timeout helper built atop `TickCounter`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Timeout {
    start: u64,
    duration: u64,
}

impl Timeout {
    /// Create a new timeout starting at `now` with `duration` ticks.
    #[inline]
    pub const fn new(now: u64, duration: u64) -> Self {
        Self { start: now, duration }
    }
    /// Whether the timeout has elapsed.
    #[inline]
    pub const fn elapsed(&self, now: u64) -> bool {
        now.wrapping_sub(self.start) >= self.duration
    }
}

#[cfg(test)]
extern crate std;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn queue_push_pop() {
        let mut q: EventQueue<u32, 2> = EventQueue::new();
        assert!(q.is_empty());
        assert_eq!(q.capacity(), 2);
        q.push(1).unwrap();
        q.push(2).unwrap();
        assert!(q.is_full());
        assert!(q.push(3).is_err());
        assert_eq!(q.pop(), Some(1));
        assert_eq!(q.pop(), Some(2));
        assert_eq!(q.pop(), None);
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Ev { A, B }
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum St { X, Y }

    fn handler(s: St, e: &Ev) -> Transition<St> {
        match (s, e) {
            (St::X, Ev::A) => Transition::to(St::Y),
            (St::Y, Ev::B) => Transition::to(St::X),
            _ => Transition::stay(),
        }
    }

    #[test]
    fn machine_steps() {
        let mut sm = StateMachine::new(St::X, handler);
        let mut q: EventQueue<Ev, 4> = EventQueue::new();
        q.push(Ev::A).unwrap();
        q.push(Ev::B).unwrap();
        sm.run(&mut q);
        assert_eq!(sm.state(), St::X);
    }

    #[test]
    fn timeout_works() {
        let mut t = TickCounter::new();
        let timeout = Timeout::new(t.value(), 3);
        assert!(!timeout.elapsed(t.value()));
        t.tick();
        t.tick();
        assert!(!timeout.elapsed(t.value()));
        t.tick();
        assert!(timeout.elapsed(t.value()));
    }
}
