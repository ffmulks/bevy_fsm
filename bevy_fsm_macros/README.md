# bevy_fsm_macros

Procedural macros for [bevy_fsm](https://crates.io/crates/bevy_fsm) - an observer-driven finite state machine framework for Bevy ECS.

## What This Crate Provides

This crate provides two derive macros:

- **`#[derive(FSMTransition)]`** - Generates a default "allow all" transition implementation
- **`#[derive(FSMState)]`** - Generates variant-specific event triggering infrastructure

**You typically don't need to add this crate directly** - it's re-exported by `bevy_fsm`.

## Usage

```rust
use bevy::prelude::*;
use bevy_fsm::{FSMState, FSMTransition, FSMPlugin};
use bevy_enum_event::EnumEvent;

// Zero boilerplate - FSMTransition derive allows all transitions
#[derive(Component, EnumEvent, FSMTransition, FSMState, Reflect, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[reflect(Component)]
enum GameState {
    MainMenu,
    Playing,
    GameOver,
}

fn setup(app: &mut App) {
    app.add_plugins(FSMPlugin::<GameState>::default());
}
```

For custom transition rules, skip the `FSMTransition` derive and implement the trait manually:

```rust
use bevy_fsm::FSMTransition;

#[derive(Component, EnumEvent, FSMState, Reflect, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum LifeFSM {
    Alive,
    Dying,
    Dead,
}

impl FSMTransition for LifeFSM {
    fn can_transition(from: Self, to: Self) -> bool {
        matches!((from, to),
            (LifeFSM::Alive, LifeFSM::Dying) |
            (LifeFSM::Dying, LifeFSM::Dead)) || from == to
    }
}
```

## Bevy Compatibility

| Bevy  | bevy_fsm_macros |
|-------|-----------------|
| 0.17  | 0.2             |
| 0.16  | 0.1             |

## Documentation

For complete documentation, examples, and best practices, see the main [bevy_fsm](https://docs.rs/bevy_fsm) crate.

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](../LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](../LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.
