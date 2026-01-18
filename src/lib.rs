pub mod watcher;
pub mod reactor;
pub mod event;
pub mod compositor;
pub mod cli;
pub mod debug;
pub mod namespaces;

// The flow of the program is as follows:
// 1. The watcher module sets up configuration watchers using cosmic-config
// 2. When a configuration change is detected, the watcher translates it into an Event and sends it to the reactor
// 3. The reactor processes the Event and invokes the appropriate IPC functions in the compositor module
// 4. The compositor module contains implementations for different compositors