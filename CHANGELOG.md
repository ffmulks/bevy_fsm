# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2025-01-25

### Added
- Initial release of `bevy_fsm` - observer-driven finite state machine framework for Bevy ECS
- `FSMPlugin` for automatic FSM setup with observer hierarchy organization
- `#[derive(FSMState)]` macro for variant-specific event generation (from `bevy_fsm_macros`)
- `#[derive(FSMTransition)]` macro for default "allow all" transition rules (from `bevy_fsm_macros`)
- `FSMTransition` trait with `can_transition` and `can_transition_ctx` methods
- `FSMState` trait for state machine infrastructure
- `fsm_observer!` macro for registering observers in organized hierarchy
- `StateChangeRequest<S>` event for triggering state transitions
- `Enter<S>`, `Exit<S>`, and `Transition<F, T>` event types
- Variant-specific events via integration with `bevy_enum_event`
- `FSMOverride` component for per-entity transition configuration
- Priority-based validation system (config wins over type-level rules)
- Support for whitelist, blacklist, allow-all, and deny-all modes
- Context-aware validation with world access
- Automatic observer hierarchy: `FSMObservers/MyFSM/observer_name`
- Initial state support with automatic `Enter` events on FSM component addition
- Full N×N transition event coverage between all state variants
- Comprehensive test suite (14 unit tests + 10 doc tests)
- Working examples: `basic`, `simple`, `fully_connected`, `transition_rules`
- Inline documentation with usage examples
- README with Quick Start guide and advanced features documentation

### Features
- Observer-driven state transitions using Bevy's native observer system
- Type-safe variant-specific events (no runtime state checks needed)
- Flexible validation: per-entity (FSMOverride) and per-type (FSMTransition)
- Organized observer hierarchy for easy debugging
- Zero-boilerplate option with `#[derive(FSMTransition)]`
- Custom transition rules via manual `FSMTransition` implementation