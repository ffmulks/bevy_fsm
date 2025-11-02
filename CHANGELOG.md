# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- **BREAKING**: Migrated to Bevy 0.17
- **BREAKING**: Updated observer triggers to use `On<Event>` instead of `Trigger<Event>` (Bevy 0.17 observer API change)
- **BREAKING**: Replaced deprecated `trigger_targets` with `trigger` - events now embed entity information directly
- Updated hierarchy API to use Bevy 0.17's new parent-child relationship system
- Observer entities are now marked as `Internal` and filtered from normal queries by default
- Tests now use `query_filtered::<EntityRef, Allow<Internal>>()` to include observer entities in hierarchy checks

### Fixed

- Fixed test `fsm_observer_macro_registers_and_organizes` to work with Bevy 0.17's internal entity filtering
- Updated deprecated `iter_entities()` usage to use recommended `query::<EntityRef>()` pattern

### Documentation

- Updated README to clarify that all events (`Enter`, `Exit`, `Transition`, `StateChangeRequest`) include an `entity` field
- Updated README testing example to use `trigger` instead of deprecated `trigger_targets`
- Added note explaining `EntityEvent` implementation and `trigger.target()` usage

## [0.1.0] - Previous Release

Initial release with Bevy 0.16 support.
