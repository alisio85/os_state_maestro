Usage Guide
===========

Defining a Machine
------------------
1. Create `enum` types for your states and events.
2. Implement a handler `fn(State, &Event) -> Transition<State>`.
3. Construct `StateMachine::new(initial_state, handler)`.
4. Use `EventQueue<Ev, N>` to buffer incoming events.

Scheduling
----------
- Drive the machine from your main loop or scheduler tick.
- Use multiple queues for different priorities if needed.

Timeouts
--------
- Advance a `TickCounter` regularly (e.g., via a timer IRQ).
- Check `Timeout::elapsed(now)` inside your handler to implement time-based transitions.

Best Practices
--------------
- Make states explicit and small (e.g., `u8`-backed enums).
- Keep handlers pure; avoid side effects inside transition logic.
- Validate transitions with unit tests for every event/state pair.
- Size queues to expected burst traffic; avoid drop-on-full if critical.
