# Cosmolith

Cosmolith is a Rust daemon that watches COSMIC configuration changes and applies | saves them live to the running Wayland compositor via IPC calls. It translates COSMIC settings into compositor-specific IPC commands so changes take effect immediately without restarting your session.

## Status [ WIP ]

## Architecture

Cosmolith is structured as a pipeline that listens to COSMIC configuration changes, normalizes them into typed events, and dispatches them to compositor backends.

- **Watcher:** Subscribes to COSMIC namespaces (for example, `com.system76.*`) and receives raw configuration changes. Each watcher is responsible for a domain (input, keybindings, workspaces, etc.) and produces typed diffs instead of compositor-specific commands.
- **Event:** A unified event model that represents all COSMIC changes (input, keybindings, workspace changes, window rules, and more). Each event is a typed payload that describes *what changed*, independent of any compositor.
- **Dispatcher:** Routes events to the active compositor and ensures only supported events are applied. It is the boundary where generic events become compositor actions.
- **Compositor Layer:** Defines the `Compositor` trait and a set of event-specific traits. Backends implement these traits to translate events into compositor-specific IPC commands.

Compositor identification is done in `identifier.rs` and the result is used to initialize the appropriate backend in the compositor module. If the active compositor implements the relevant event-specific trait, the corresponding apply method is invoked.

**The following architecture is chosen not for extreme optimality but for modularity in use. This makes the following work compositor independent & integrable/reusable ( By just sending `Event` struct  & intializing the `reactor`)**

## Requirements

- Rust (edition 2024)
- A COSMIC session with `cosmic-config` available
- Supported compositor backend in tree:
  - Hyprland
  - Sway
  - GNOME input mapping is partial and currently limited to mouse/touchpad paths

## Build

```sh
cargo build
```

## Run

```sh
cargo run
```

Cosmolith will print detected compositor information, apply the current COSMIC keyboard state once
at startup on keyboard-capable backends such as Hyprland and Sway, and then continue applying
configuration updates as changes are observed.

## Contributing

1. Fork the repository.
2. Create a feature branch.
3. Make changes with focused commits.
4. Open a pull request describing the change and rationale.

## License
**MIT LICENSE**
