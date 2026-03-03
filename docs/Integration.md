Integration Notes
=================

Kernel Subsystems
-----------------
- Drivers: model probe->init->ready->suspend->resume sequences.
- Boot: orchestrate multi-phase boot and handoff steps.
- Power: represent power states and transitions per device class.
- Tasks: implement lifecycle states (new, ready, running, blocked, terminated).

Interrupts
----------
- Push minimal events in IRQ context; process in non-IRQ contexts.
- Keep handlers short and non-blocking.

Memory
------
- All structures are stack/static allocated; choose capacities consciously.
- No heap; no allocations.

Diagnostics
-----------
- Wrap handlers to capture transition traces for post-mortem analysis.
- Expose state snapshots via debug interfaces where applicable.
