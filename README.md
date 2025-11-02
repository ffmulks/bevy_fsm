# bevy_fsm

Observer-driven finite state machine framework for Bevy ECS.

## Bevy Compatibility

|  Bevy   | bevy_fsm |
|---------|----------|
| 0.17    | main     |
| 0.16    | 0.1.0    |

## Features

- **Enum-based states**: Keep your states as simple enum variants
- **Observer-driven**: React to state changes via Bevy observers
- **Variant-specific events**: No runtime state checks needed in observers
- **Flexible validation**: Per-entity and per-type transition rules
- **Clean API**: FSMPlugin for automatic setup
- **Initial state support**: Automatic enter events when FSM components are added
- **Organized hierarchy**: Observers automatically organized in entity hierarchy

## Quick Start

```rust
use bevy::prelude::*;
use bevy_fsm::{FSMState, FSMTransition, FSMPlugin, StateChangeRequest, Enter, Exit, Transition, fsm_observer};
use bevy_enum_event::EnumEvent;

fn plugin(app: &mut App) {
    // FSMPlugin automatically sets up the observer hierarchy on first use
    app.add_plugins(FSMPlugin::<LifeFSM>::default());

    // Use fsm_observer! macro for variant-specific observers
    // This is functionally identical to a typed global observer but gets automatically parented
    // into a custom FSMObservers/LifeFSM/on_enter_dying hierarchy that keeps observers nicely
    // sorted by their respective FSM.
    fsm_observer!(app, LifeFSM, on_enter_dying);
    fsm_observer!(app, LifeFSM, on_exit_alive);
    fsm_observer!(app, LifeFSM, on_transition_dying_dead);
}

#[derive(Component, EnumEvent, FSMState, Reflect, Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[reflect(Component)]
enum LifeFSM {
    Alive,
    Dying,
    Dead,
}

impl FSMTransition for LifeFSM {
    // This is used as baseline filter to allow and forbid transitions
    fn can_transition(from: Self, to: Self) -> bool {
        matches!((from, to),
            (LifeFSM::Alive, LifeFSM::Dying) |
            (LifeFSM::Dying, LifeFSM::Alive) |
            (LifeFSM::Dying, LifeFSM::Dead)) || from == to
    }
}

#[derive(Component)]
struct DyingAnimation;

fn on_enter_dying(trigger: On<Enter<life_fsm::Dying>>, mut commands: Commands) {
    let entity = trigger.event().entity;
    commands.entity(entity).insert(DyingAnimation);
}

fn on_exit_alive(trigger: On<Exit<life_fsm::Alive>>) {
    let entity = trigger.event().entity;
    println!("Entity {} was unalived.", entity);
}

fn on_transition_dying_dead(
    trigger: On<Transition<life_fsm::Dying, life_fsm::Alive>>,
    mut commands: Commands
) {
    let entity = trigger.event().entity;
    println!("Entity {} was saved from the brink of death.", entity);
}
```

## Core Concepts

### FSMTransition Trait

Implement this trait to define which state transitions are valid:

```rust
impl FSMTransition for MyFSM {
    fn can_transition(from: Self, to: Self) -> bool {
        // Define your transition rules
        matches!((from, to),
            (MyFSM::StateA, MyFSM::StateB) |
            (MyFSM::StateB, MyFSM::StateC)) || from == to
    }

    // Optional: context-aware validation with world access
    fn can_transition_ctx(world: &World, entity: Entity, from: Self, to: Self) -> bool {
        if !Self::can_transition(from, to) {
            return false;
        }
        // Additional validation using world state
        world.get::<SomeComponent>(entity).is_some()
    }
}
```

### EnumEvent and FSMState Derives

**bevy_fsm** uses two derive macros from `bevy_enum_event`:

1. **`#[derive(EnumEvent)]`** - Generates variant-specific event types in a `modulename::Variant` hierarchy
2. **`#[derive(FSMState)]`** - Implements FSM-specific trigger methods for Enter/Exit/Transition events

Together they enable:

- Type-safe variant-specific events
- Automatic Enter/Exit event triggering
- Full N×N transition event support

```rust
use bevy_enum_event::{EnumEvent, FSMState};

#[derive(Component, EnumEvent, FSMState, Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum BlockFSM {
    Tile,    // Generates blockfsm::Tile event type
    Loose,   // Generates blockfsm::Loose event type
    Disabled // Generates blockfsm::Disabled event type
}

impl FSMState for BlockFSM {}

// Use with Enter/Exit wrappers:
fn on_tile_enter(enter: On<Enter<blockfsm::Tile>>, ...) { }
fn on_tile_exit(exit: On<Exit<blockfsm::Tile>>, ...) { }
```

### FSMPlugin - Automatic Setup

The easiest way to register an FSM is with `FSMPlugin`:

```rust
use bevy_fsm::FSMPlugin;

fn plugin(app: &mut App) {
    // Automatically registers apply_state_request and on_fsm_added observers
    app.add_plugins(FSMPlugin::<MyFSM>::default());

    // Optional: Skip automatic on_fsm_added observer
    app.add_plugins(FSMPlugin::<MyFSM>::new().ignore_fsm_addition());
}
```

### fsm_observer! Macro

Use the `fsm_observer!` macro to register variant-specific observers with automatic hierarchy organization:

```rust
use bevy_fsm::{fsm_observer, Enter};

fn on_enter_loose(trigger: On<Enter<blockfsm::Loose>>, mut commands: Commands) {
    let entity = trigger.event().entity;
    commands.entity(entity).insert(RigidBody::Dynamic);
}

fn plugin(app: &mut App) {
    app.add_plugins(FSMPlugin::<BlockFSM>::default());

    // Registers and organizes observers in entity hierarchy
    fsm_observer!(app, BlockFSM, on_enter_loose);
    fsm_observer!(app, BlockFSM, on_exit_loose);
}
```

### Manual Observer Registration

If you prefer manual control, you can register observers directly:

```rust
use bevy_fsm::{apply_state_request, on_fsm_added};

// Handles state transition requests
app.world_mut().add_observer(apply_state_request::<MyFSM>);

// Triggers enter events when FSM is first added to entity
app.world_mut().add_observer(on_fsm_added::<MyFSM>);

// Variant-specific observers
app.world_mut().add_observer(on_enter_loose);
```

### Generic Event Observers

You can also observe generic events if you need runtime state checking:

```rust
fn on_any_enter(
    trigger: On<Enter<BlockFSM>>,
    mut commands: Commands,
) {
    let state = trigger.event().state;
    match state {
        BlockFSM::Tile => { /* handle tile */ },
        BlockFSM::Loose => { /* handle loose */ },
        _ => {}
    }
}
```

## Advanced Features

### Per-Entity Configuration with Priority Model

FSMOverride allows per-entity transition control with a **priority-based system**: config takes precedence over FSMTransition rules.

#### Priority Principle: Config Wins, Rules Fill Gaps

- **Whitelist**: Transitions ON the list are **immediately accepted** (config wins)
- **Whitelist**: Transitions NOT on the list check FSMTransition if `with_rules()` is used, else denied
- **Blacklist**: Transitions ON the list are **immediately denied** (config wins)
- **Blacklist**: Transitions NOT on the list check FSMTransition if `with_rules()` is used, else accepted

```rust
use bevy_fsm::FSMOverride;

// Example 1: Force allow specific transition (override FSMTransition)
commands.entity(special_npc).insert((
    AnimationState::Idling,
    FSMOverride::whitelist([
        (AnimationState::Idling, AnimationState::Flying), // Normally forbidden
    ]),
));
// Idling->Flying: ACCEPT (whitelisted, config wins)
// Idling->Walking: DENY (not whitelisted)

// Example 2: Whitelist + fallback to FSMTransition for others
commands.entity(npc).insert((
    AnimationState::Idling,
    FSMOverride::whitelist([
        (AnimationState::Idling, AnimationState::Flying), // Force allow
    ]).with_rules(),
));
// Idling->Flying: ACCEPT (whitelisted, config wins)
// Idling->Walking: Check FSMTransition (not whitelisted, rules fill gap)

// Example 3: Force deny specific transition
commands.entity(injured_npc).insert((
    AnimationState::Idling,
    FSMOverride::blacklist([
        (AnimationState::Idling, AnimationState::Running), // Prevent running
    ]),
));
// Idling->Running: DENY (blacklisted, config wins)
// Idling->Walking: ACCEPT (not blacklisted)

// Example 4: Blacklist + fallback to FSMTransition for others
commands.entity(npc).insert((
    AnimationState::Idling,
    FSMOverride::blacklist([
        (AnimationState::Idling, AnimationState::Running),
    ]).with_rules(),
));
// Idling->Running: DENY (blacklisted, config wins)
// Idling->Walking: Check FSMTransition (not blacklisted, rules fill gap)
```

#### FSMOverride Modes

- **`whitelist([...])`**: Only listed transitions pass immediately. Others denied unless `with_rules()` is used.
- **`blacklist([...])`**: Listed transitions denied immediately. Others allowed unless `with_rules()` is used.
- **`allow_all()`**: All transitions pass (bypass FSMTransition unless `with_rules()` is used).
- **`deny_all()`**: All transitions denied (immutable state).

#### Using with_rules()

The `with_rules()` method enables FSMTransition validation for transitions NOT decided by the config:

```rust
// Without with_rules: whitelist is sole authority
FSMOverride::whitelist([(State::A, State::C)])
// A->C: ACCEPT (whitelisted)
// A->B: DENY (not whitelisted)

// With with_rules: whitelist wins, FSMTransition fills gaps
FSMOverride::whitelist([(State::A, State::C)]).with_rules()
// A->C: ACCEPT (whitelisted, FSMTransition NOT checked)
// A->B: Check FSMTransition (not whitelisted, rules enabled)
```

### Context-Aware Validation

Use world state in transition validation:

```rust
impl FSMTransition for AnimationState {
    fn can_transition_ctx(world: &World, entity: Entity, from: Self, to: Self) -> bool {
        if !Self::can_transition(from, to) {
            return false;
        }

        // Verify animation exists for target state
        if let Some(animation) = world.get::<SpriteAnimation>(entity) {
            animation.has_state(to)
        } else {
            false
        }
    }
}
```

## Event Types

Each FSM generates several event types. All transition events implement `EntityEvent` and contain an `entity` field to identify the target entity:

- `StateChangeRequest<S>`: Request to change an entity's state (contains `entity` and `next` fields)
- `Enter<S>`: Generic enter event (contains `entity` and `state` fields)
- `Exit<S>`: Generic exit event (contains `entity` and `state` fields)
- `Transition<S, S>`: Generic transition event (contains `entity`, `from`, and `to` fields)

The states themselves generate standard events. They are usually unit events without data.

- `modulename::Variant`: Type-safe variant event types (used with `Enter<T>` and `Exit<T>` wrappers)

In observer functions, access the entity via `trigger.event().entity`.

## How It Works

When a state change is requested:

1. `apply_state_request` observer validates the transition
2. Exit events are triggered: `Exit<S>` (generic) and `Exit<modulename::Variant>` (type-safe)
3. Transition event is triggered: `Transition<S, S>` with `from` and `to` fields
4. State component is updated on the entity
5. Enter events are triggered: `Enter<S>` (generic) and `Enter<modulename::Variant>` (type-safe)

When an FSM component is first added:

1. `on_fsm_added` observer detects the new component
2. Enter events are triggered for the initial state

## Best Practices

- **Use FSMPlugin** for automatic FSM setup (recommended)
- **Use fsm_observer! macro** for registering observers with automatic hierarchy organization
- **Use variant-specific observers** for cleaner code without state checks
- **Keep transition logic simple** in `can_transition`
- **Use context validation** (`can_transition_ctx`) for world-dependent rules
- **Derive FSMState and Reflect** together for full functionality
- **Use snake_case** when accessing generated modules (e.g., `Enter<lifefsm::Dying>`)
- **Import Enter and Exit** from `bevy_fsm` when using variant-specific observers

## Migration from Bevy 0.16 to 0.17

1. **Observer parameter type**: Change `Trigger<Event>` to `On<Event>`

   ```rust
   // Old (Bevy 0.16):
   fn my_observer(trigger: Trigger<Enter<MyState>>) { }

   // New (Bevy 0.17):
   fn my_observer(trigger: On<Enter<MyState>>) { }
   ```

2. **Accessing the target entity**: Change `trigger.target()` to `trigger.event().entity`

   ```rust
   // Old (Bevy 0.16):
   let entity = trigger.target();

   // New (Bevy 0.17):
   let entity = trigger.event().entity;
   ```

3. **Triggering events**: Use `trigger()` instead of `trigger_targets()`, and include the entity in the event struct

   ```rust
   // Old (Bevy 0.16):
   commands.trigger_targets(
       StateChangeRequest { next: MyState::NewState },
       entity
   );

   // New (Bevy 0.17):
   commands.trigger(
       StateChangeRequest { entity, next: MyState::NewState }
   );
   ```

## Important: Timing of Initial Enter Events

**WARNING**: When an FSM component is added during entity spawn, the initial `Enter` event fires **in the same frame**, before the entity is fully initialized.

### What This Means

```rust
let entity = commands.spawn((
    LifeFSM::Alive,  // Enter event fires immediately!
    Health::new(100),
    // Other components...
)).id();
```

When this spawn occurs:

1. FSM component is added
2. `on_fsm_added` observer fires **immediately**
3. `Enter<life_fsm::Alive>` event is triggered
4. **Other components may not exist yet!**
5. Child entities are not spawned yet
6. Asset handles may not be loaded

**Consider using `ignore_fsm_addition()`** if you don't need initial Enter events:

   ```rust
   app.add_plugins(FSMPlugin::<LifeFSM>::new().ignore_fsm_addition());
   ```

## Testing

```rust
use bevy_fsm::{FSMPlugin, fsm_observer};

#[test]
fn test_state_transition() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);

    // Register FSM using FSMPlugin
    app.add_plugins(FSMPlugin::<LifeFSM>::default());
    fsm_observer!(app, LifeFSM, on_dying);

    // Spawn entity with initial state
    let entity = app.world_mut().spawn(LifeFSM::Alive).id();
    app.update(); // Triggers on_fsm_added

    // Request transition
    app.world_mut().commands().trigger(
        StateChangeRequest::<LifeFSM> { entity, next: LifeFSM::Dying },
    );
    app.update();

    // Verify transition occurred
    assert_eq!(*app.world().get::<LifeFSM>(entity).unwrap(), LifeFSM::Dying);
}
```

## Module Structure

```markdown
bevy_fsm/
├── src/lib.rs           # Core traits and observer functions
├── Cargo.toml
└── README.md

bevy_enum_event/        # Separate crate (dependency)
├── src/lib.rs           # EnumEvent and FSMState derive macros
├── Cargo.toml
└── README.md
```

**Note**: `bevy_fsm` depends on `bevy_enum_event` with the `fsm` feature enabled.

## AI Disclaimer

- Refactoring and documentation supported by Claude Code
- Minor editing supported by ChatGPT Codex
- The process and final releases are thoroughly supervised and checked by the author

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
