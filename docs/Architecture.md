Architecture
============

Components
----------
- Transition<S>: pure transition outcome.
- EventQueue<T, N>: bounded FIFO for events.
- StateMachine<S, E, H>: deterministic executor with user handler.
- TickCounter: monotonic tick counter.
- Timeout: elapsed-check helper.

Data Flow
---------
- Producers push events into `EventQueue`.
- The main loop drains the queue and feeds events to `StateMachine`.
- The handler returns `Transition` which updates the state.
- Optional timeouts influence decisions using `TickCounter`.

Constraints
-----------
- No allocations; capacity is fixed via const generics.
- No unsafe; all operations are safe Rust.
- `no_std` default; `std` only in tests and docs examples.

Extensibility
-------------
- Compose multiple machines to model subsystems.
- Use separate queues per priority class.
- Wrap handlers to add tracing or metrics.
