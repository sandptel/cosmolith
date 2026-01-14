use serde;

// The following is an internal representation of various events that can occur via config changes
// The watcher module translates config change notifications into these events
// The Reactor Module then processes these events accordingly by calling appropriate ipc functions for different compositors
// Strict Rules: These Events will be atomic and represent a single change only
// These Events would be extremely simple ( e..g ToggleTouchpad, SetTouchpadSensitivity(u8) etc )
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Event {

}