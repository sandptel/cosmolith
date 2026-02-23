use cosmic_comp_config::{XkbConfig, KeyboardConfig, NumlockState};
use cosmic_comp_config::input::{
    AccelConfig, ClickMethod, DeviceState, InputConfig, ScrollConfig, ScrollMethod, TapButtonMap,
    TapConfig,
};

use super::Event;

/// Extension trait for diffing `InputConfig` to produce touchpad/mouse events.
pub trait InputConfigDiff {
    /// Compare self (old) with new config and return touchpad-related events.
    fn from_touchpad(&self, new: &InputConfig) -> Vec<Event>;
    /// Compare self (old) with new config and return mouse-related events.
    fn from_mouse(&self, new: &InputConfig) -> Vec<Event>;
}

/// Extension trait for diffing `XkbConfig` to produce keyboard events.
pub trait XkbConfigDiff {
    /// Compare self (old) with new config and return keyboard-related events.
    fn from(&self, new: &XkbConfig) -> Vec<Event>;
}

/// Extension trait for diffing `KeyboardConfig` to produce keyboard events.
pub trait KeyboardConfigDiff {
    /// Compare self (old) with new config and return keyboard-related events (numlock).
    fn from(&self, new: &KeyboardConfig) -> Vec<Event>;
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    TouchPad(TouchpadEvent),
    Mouse(MouseEvent),
    Keyboard(KeyboardEvent),
}

#[derive(Clone, Debug, PartialEq)]
pub enum KeyboardEvent {
    /// XKB rules file.
    Rules(String),
    /// Keyboard model.
    Model(String),
    /// Keyboard layout(s).
    Layout(String),
    /// Keyboard variant(s).
    Variant(String),
    /// XKB options.
    Options(Option<String>),
    /// Key repeat delay in ms.
    RepeatDelay(u32),
    /// Key repeat rate in Hz.
    RepeatRate(u32),
    /// Numlock state.
    /// NumlockState::BootOn | BootOff | LastBoot.
    NumLock(NumlockState),
}

#[derive(Clone, Debug, PartialEq)]
pub enum TouchpadEvent {
    /// Touchpad enable state.
    /// DeviceState::Enabled = on, Disabled = off, DisabledOnExternalMouse = auto-off with external mouse.
    State(DeviceState),
    /// Acceleration settings.
    /// profile: AccelProfile::Flat | AccelProfile::Adaptive.
    Acceleration(Option<AccelConfig>),
    /// Calibration matrix for touchpad coordinates.
    Calibration(Option<[f32; 6]>),
    /// Click method.
    /// ClickMethod::ButtonAreas | ClickMethod::Clickfinger.
    ClickMethod(Option<ClickMethod>),
    /// Disable while typing.
    /// true = ignore touchpad while typing, false = always active.
    DisableWhileTyping(Option<bool>),
    /// Left-handed mode.
    /// true = swap button mapping for left-handed use.
    LeftHanded(Option<bool>),
    /// Middle button emulation.
    /// true = emulate middle click (usually by left+right click).
    MiddleButtonEmulation(Option<bool>),
    /// Rotation angle in degrees.
    RotationAngle(Option<u32>),
    /// Scroll configuration.
    /// ScrollMethod::NoScroll | TwoFinger | Edge | OnButtonDown.
    ///
    /// TODO: Redundant when all sub-field events (ScrollMethod/NaturalScroll/ScrollFactor/ScrollButton)
    /// are emitted. IPC handlers should ignore this if equivalent fine-grained events are present.
    ScrollConfig(Option<ScrollConfig>),
    /// Tap configuration.
    /// TapButtonMap::LeftRightMiddle | LeftMiddleRight.
    ///
    /// TODO: Redundant when all sub-field events (TapEnabled/TapButtonMap/TapDrag/TapDragLock)
    /// are emitted. IPC handlers should ignore this if equivalent fine-grained events are present.
    TapConfig(Option<TapConfig>),
    /// Map to output name (display ID).
    MapToOutput(Option<String>),

    /// Scroll method only.
    ScrollMethod(Option<ScrollMethod>),
    /// Natural scroll.
    /// true = natural (content follows fingers), false = traditional.
    NaturalScroll(Option<bool>),
    /// Scroll factor / speed multiplier.
    ScrollFactor(Option<f64>),
    /// Scroll button for OnButtonDown mode.
    ScrollButton(Option<u32>),

    /// Tap enabled.
    /// true = tapping generates clicks, false = no tap-to-click.
    TapEnabled(bool),
    /// Tap button map.
    /// TapButtonMap::LeftRightMiddle | LeftMiddleRight.
    TapButtonMap(Option<TapButtonMap>),
    /// Tap drag enabled.
    /// true = tap-and-drag allowed, false = disabled.
    TapDrag(bool),
    /// Tap drag lock.
    /// true = drag lock enabled, false = disabled.
    TapDragLock(bool),
}

#[derive(Clone, Debug, PartialEq)]
pub enum MouseEvent {
    /// Mouse enable state.
    /// DeviceState::Enabled = on, Disabled = off, DisabledOnExternalMouse = auto-off with external mouse.
    State(DeviceState),
    /// Acceleration settings.
    /// profile: AccelProfile::Flat | AccelProfile::Adaptive.
    Acceleration(Option<AccelConfig>),
    /// Calibration matrix for mouse coordinates.
    Calibration(Option<[f32; 6]>),
    /// Click method.
    /// ClickMethod::ButtonAreas | ClickMethod::Clickfinger.
    ClickMethod(Option<ClickMethod>),
    /// Disable while typing.
    /// true = ignore device while typing, false = always active.
    DisableWhileTyping(Option<bool>),
    /// Left-handed mode.
    /// true = swap button mapping for left-handed use.
    LeftHanded(Option<bool>),
    /// Middle button emulation.
    /// true = emulate middle click (usually by left+right click).
    MiddleButtonEmulation(Option<bool>),
    /// Rotation angle in degrees.
    RotationAngle(Option<u32>),
    /// Scroll configuration.
    /// ScrollMethod::NoScroll | TwoFinger | Edge | OnButtonDown.
    ///
    /// TODO: Redundant when all sub-field events (ScrollMethod/NaturalScroll/ScrollFactor/ScrollButton)
    /// are emitted. IPC handlers should ignore this if equivalent fine-grained events are present.
    ScrollConfig(Option<ScrollConfig>),
    /// Tap configuration.
    /// TapButtonMap::LeftRightMiddle | LeftMiddleRight.
    ///
    /// TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
    /// if equivalent fine-grained events are present.
    TapConfig(Option<TapConfig>),
    /// Map to output name (display ID).
    MapToOutput(Option<String>),

    /// Scroll method only.
    ScrollMethod(Option<ScrollMethod>),
    /// Natural scroll.
    /// true = natural (content follows fingers), false = traditional.
    NaturalScroll(Option<bool>),
    /// Scroll factor / speed multiplier.
    ScrollFactor(Option<f64>),
    /// Scroll button for OnButtonDown mode.
    ScrollButton(Option<u32>),
}

impl InputConfigDiff for InputConfig {
    fn from_touchpad(&self, new: &InputConfig) -> Vec<Event> {
        if self == new {
            return vec![];
        }

        let mut events = Vec::new();

        if self.state != new.state {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::State(new.state)));
            events.push(event);
        }
        if self.acceleration != new.acceleration {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::Acceleration(
                new.acceleration.clone(),
            )));
            events.push(event);
        }
        if self.calibration != new.calibration {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::Calibration(
                new.calibration,
            )));
            events.push(event);
        }
        if self.click_method != new.click_method {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::ClickMethod(
                new.click_method,
            )));
            events.push(event);
        }
        if self.disable_while_typing != new.disable_while_typing {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::DisableWhileTyping(
                new.disable_while_typing,
            )));
            events.push(event);
        }
        if self.left_handed != new.left_handed {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::LeftHanded(
                new.left_handed,
            )));
            events.push(event);
        }
        if self.middle_button_emulation != new.middle_button_emulation {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::MiddleButtonEmulation(
                new.middle_button_emulation,
            )));
            events.push(event);
        }
        if self.rotation_angle != new.rotation_angle {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::RotationAngle(
                new.rotation_angle,
            )));
            events.push(event);
        }
        if self.scroll_config != new.scroll_config {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::ScrollConfig(
                new.scroll_config.clone(),
            )));

            // TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
            // if equivalent fine-grained events are present.
            events.push(event);

            if let (Some(old_scroll), Some(new_scroll)) =
                (&self.scroll_config, &new.scroll_config)
            {
                if old_scroll.method != new_scroll.method {
                    let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::ScrollMethod(
                        new_scroll.method,
                    )));
                    events.push(event);
                }
                if old_scroll.natural_scroll != new_scroll.natural_scroll {
                    let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::NaturalScroll(
                        new_scroll.natural_scroll,
                    )));
                    events.push(event);
                }
                if old_scroll.scroll_button != new_scroll.scroll_button {
                    // Unreachable: cosmic-settings currently does not produce this event
                    let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::ScrollButton(
                        new_scroll.scroll_button,
                    )));
                    events.push(event);
                }
                if old_scroll.scroll_factor != new_scroll.scroll_factor {
                    let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::ScrollFactor(
                        new_scroll.scroll_factor,
                    )));
                    events.push(event);
                }
            }
        }

        if self.tap_config != new.tap_config {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::TapConfig(
                new.tap_config.clone(),
            )));

            // TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
            // if equivalent fine-grained events are present.
            events.push(event);

            if let (Some(old_tap), Some(new_tap)) = (&self.tap_config, &new.tap_config) {
                if old_tap.enabled != new_tap.enabled {
                    let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::TapEnabled(
                        new_tap.enabled,
                    )));
                    events.push(event);
                }
                if old_tap.button_map != new_tap.button_map {
                    // Unreachable: cosmic-settings currently does not produce this event
                    let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::TapButtonMap(
                        new_tap.button_map,
                    )));
                    events.push(event);
                }
                if old_tap.drag != new_tap.drag {
                    // Unreachable: cosmic-settings currently does not produce this event
                    let event =
                        Event::Input(InputEvent::TouchPad(TouchpadEvent::TapDrag(new_tap.drag)));
                    events.push(event);
                }
                if old_tap.drag_lock != new_tap.drag_lock {
                    // Unreachable: cosmic-settings currently does not produce this event
                    let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::TapDragLock(
                        new_tap.drag_lock,
                    )));
                    events.push(event);
                }
            }
        }
        if self.map_to_output != new.map_to_output {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::MapToOutput(
                new.map_to_output.clone(),
            )));
            events.push(event);
        }

        events
    }

    fn from_mouse(&self, new: &InputConfig) -> Vec<Event> {
        if self == new {
            return vec![];
        }

        let mut events = Vec::new();

        if self.state != new.state {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::State(new.state)));
            events.push(event);
        }
        if self.acceleration != new.acceleration {
            let event = Event::Input(InputEvent::Mouse(MouseEvent::Acceleration(
                new.acceleration.clone(),
            )));
            events.push(event);
        }
        if self.calibration != new.calibration {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::Calibration(new.calibration)));
            events.push(event);
        }
        if self.click_method != new.click_method {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::ClickMethod(new.click_method)));
            events.push(event);
        }
        if self.disable_while_typing != new.disable_while_typing {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::DisableWhileTyping(
                new.disable_while_typing,
            )));
            events.push(event);
        }
        if self.left_handed != new.left_handed {
            let event = Event::Input(InputEvent::Mouse(MouseEvent::LeftHanded(new.left_handed)));
            events.push(event);
        }
        if self.middle_button_emulation != new.middle_button_emulation {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::MiddleButtonEmulation(
                new.middle_button_emulation,
            )));
            events.push(event);
        }
        if self.rotation_angle != new.rotation_angle {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::RotationAngle(
                new.rotation_angle,
            )));
            events.push(event);
        }
        if self.scroll_config != new.scroll_config {
            let event = Event::Input(InputEvent::Mouse(MouseEvent::ScrollConfig(
                new.scroll_config.clone(),
            )));

            // TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
            // if equivalent fine-grained events are present.
            events.push(event);

            if let (Some(old_scroll), Some(new_scroll)) =
                (&self.scroll_config, &new.scroll_config)
            {
                if old_scroll.method != new_scroll.method {
                    // Unreachable: cosmic-settings currently does not produce this event
                    let event = Event::Input(InputEvent::Mouse(MouseEvent::ScrollMethod(
                        new_scroll.method,
                    )));
                    events.push(event);
                }
                if old_scroll.natural_scroll != new_scroll.natural_scroll {
                    let event = Event::Input(InputEvent::Mouse(MouseEvent::NaturalScroll(
                        new_scroll.natural_scroll,
                    )));
                    events.push(event);
                }
                if old_scroll.scroll_button != new_scroll.scroll_button {
                    // Unreachable: cosmic-settings currently does not produce this event
                    let event = Event::Input(InputEvent::Mouse(MouseEvent::ScrollButton(
                        new_scroll.scroll_button,
                    )));
                    events.push(event);
                }
                if old_scroll.scroll_factor != new_scroll.scroll_factor {
                    let event = Event::Input(InputEvent::Mouse(MouseEvent::ScrollFactor(
                        new_scroll.scroll_factor,
                    )));
                    events.push(event);
                }
            }
        }
        if self.tap_config != new.tap_config {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::TapConfig(
                new.tap_config.clone(),
            )));

            // TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
            // if equivalent fine-grained events are present.
            events.push(event);
        }
        if self.map_to_output != new.map_to_output {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::MapToOutput(
                new.map_to_output.clone(),
            )));
            events.push(event);
        }

        events
    }
}

impl XkbConfigDiff for XkbConfig {
    fn from(&self, new: &XkbConfig) -> Vec<Event> {
        if self == new {
            return vec![];
        }

        let mut events = Vec::new();

        if self.rules != new.rules {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Rules(
                new.rules.clone(),
            )));
            events.push(event);
        }
        if self.model != new.model {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Model(
                new.model.clone(),
            )));
            events.push(event);
        }
        if self.layout != new.layout {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Layout(
                new.layout.clone(),
            )));
            events.push(event);
        }
        if self.variant != new.variant {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Variant(
                new.variant.clone(),
            )));
            events.push(event);
        }
        if self.options != new.options {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Options(
                new.options.clone(),
            )));
            events.push(event);
        }
        if self.repeat_delay != new.repeat_delay {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::RepeatDelay(
                new.repeat_delay,
            )));
            events.push(event);
        }
        if self.repeat_rate != new.repeat_rate {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::RepeatRate(
                new.repeat_rate,
            )));
            events.push(event);
        }

        events
    }
}

impl KeyboardConfigDiff for KeyboardConfig {
    fn from(&self, new: &KeyboardConfig) -> Vec<Event> {
        if self == new {
            return vec![];
        }

        let mut events = Vec::new();

        if self.numlock_state != new.numlock_state {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::NumLock(
                new.numlock_state,
            )));
            events.push(event);
        }

        events
    }
}