use serde;

// The following is an internal representation of various events that can occur via config changes
// The watcher module translates config change notifications into these events
// The Reactor Module then processes these events accordingly by calling appropriate ipc functions for different compositors
// Strict Rules: These Events will be atomic and represent a single change only
// These Events would be extremely simple ( e..g ToggleTouchpad, SetTouchpadSensitivity(u8) etc )
#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum Event {
    Input(InputEvent),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum InputEvent {
    Touchpad(TouchpadEvent),
    Keyboard(KeyboardEvent),
    Mouse(MouseEvent),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum TouchpadEvent {
    SetLeftHanded(bool),
    SetAccelerationSpeed(f64),
    // Fix LAter: Was not able to find changes being noticed in this com.system76.CosmicComp 
    // ( maybe its not implemented ? ) ( The descriptions say: Automatically adjust tracking sensitivity based speed)
    // SetEnableTouchpadAcceleration(bool),
    SetDisableWhileTyping(bool),
    // Secon
    SetTapEnabled(bool),
    SetScrollFactor(f64),
    SetNaturalScroll(bool),
    SetState(Option<bool>),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum KeyboardEvent {
    ToggleKeyboardBacklight,
    SetKeyboardBacklightLevel(u8),
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub enum MouseEvent {
    ToggleMouseAcceleration,
    SetMouseSensitivity(u8),
}
