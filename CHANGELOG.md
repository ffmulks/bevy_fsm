# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.3.0] - 2025-01-20

### Changed

- **BREAKING**: Migrated to Bevy 0.18
- **BREAKING**: `EntityEvent::event_target_mut` removed (moved to `SetEntityEventTarget` trait in Bevy 0.18)
- **BREAKING**: Use `trigger.event_target()` instead of `trigger.target()` to access entity in observers
- Updated all documentation examples to use Bevy 0.18 patterns
- Observer entities no longer marked as `Internal` (component removed in Bevy 0.18)

### Fixed

- Tests updated to work without `Internal` component filtering

## [0.2.0] - 2025-10-01

### Changed

- **BREAKING**: Migrated to Bevy 0.17
- **BREAKING**: Observer triggers now use `On<Event>` instead of `Trigger<Event>`
- **BREAKING**: Events now embed entity field, use `trigger()` not `trigger_targets()`
- Updated hierarchy API for Bevy 0.17
- Observer entities marked as `Internal`
- Tests updated for internal entity filtering

## [0.1.0] - 2025-09-15

### Added

- Initial release for Bevy 0.16
- Observer-driven FSM framework
- `FSMState` and `FSMTransition` derive macros
- `FSMPlugin` for automatic observer setup
- `fsm_observer!` macro for hierarchy organization
- `FSMOverride` component for per-entity transition rules
- Enter, Exit, and Transition events
- Variant-specific event types via `EnumEvent`
- Context-aware transition validation with world access
