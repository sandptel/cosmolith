use cosmic_comp_config::{XkbConfig, KeyboardConfig, NumlockState};
use cosmic_comp_config::input::{
    AccelConfig, ClickMethod, DeviceState, InputConfig, ScrollConfig, ScrollMethod, TapButtonMap,
    TapConfig,
};

use super::Event;

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

impl TouchpadEvent {
    // #todo: convert it to a &self methods pub fn from(&self, new: InputConfig) -> Vec<Event> where &self is the old config
    // I am unable to decide good name so leaving it :)
    pub fn from(old: InputConfig, new: InputConfig) -> Vec<Event> {
        if old == new {
            return vec![];
        }

        let mut events = Vec::new();

        if old.state != new.state {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::State(new.state)));
            events.push(event);
        }
        if old.acceleration != new.acceleration {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::Acceleration(
                new.acceleration.clone(),
            )));
            events.push(event);
        }
        if old.calibration != new.calibration {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::Calibration(
                new.calibration,
            )));
            events.push(event);
        }
        if old.click_method != new.click_method {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::ClickMethod(
                new.click_method,
            )));
            events.push(event);
        }
        if old.disable_while_typing != new.disable_while_typing {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::DisableWhileTyping(
                new.disable_while_typing,
            )));
            events.push(event);
        }
        if old.left_handed != new.left_handed {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::LeftHanded(
                new.left_handed,
            )));
            events.push(event);
        }
        if old.middle_button_emulation != new.middle_button_emulation {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::MiddleButtonEmulation(
                new.middle_button_emulation,
            )));
            events.push(event);
        }
        if old.rotation_angle != new.rotation_angle {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::RotationAngle(
                new.rotation_angle,
            )));
            events.push(event);
        }
        if old.scroll_config != new.scroll_config {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::ScrollConfig(
                new.scroll_config.clone(),
            )));

            // TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
            // if equivalent fine-grained events are present.
            events.push(event);

            if let (Some(old_scroll), Some(new_scroll)) =
                (old.scroll_config, new.scroll_config.clone())
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

        if old.tap_config != new.tap_config {
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::TapConfig(
                new.tap_config.clone(),
            )));

            // TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
            // if equivalent fine-grained events are present.
            events.push(event);

            if let (Some(old_tap), Some(new_tap)) = (old.tap_config, new.tap_config.clone()) {
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
        if old.map_to_output != new.map_to_output {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::TouchPad(TouchpadEvent::MapToOutput(
                new.map_to_output,
            )));
            events.push(event);
        }

        events
    }
}

impl MouseEvent {
    // #todo: convert it to a &self methods pub fn from(&self, new: InputConfig) -> Vec<Event> where &self is the old config
    // I am unable to decide good name so leaving it :)
    pub fn from(old: InputConfig, new: InputConfig) -> Vec<Event> {
        if old == new {
            return vec![];
        }

        let mut events = Vec::new();

        if old.state != new.state {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::State(new.state)));
            events.push(event);
        }
        if old.acceleration != new.acceleration {
            let event = Event::Input(InputEvent::Mouse(MouseEvent::Acceleration(
                new.acceleration.clone(),
            )));
            events.push(event);
        }
        if old.calibration != new.calibration {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::Calibration(new.calibration)));
            events.push(event);
        }
        if old.click_method != new.click_method {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::ClickMethod(new.click_method)));
            events.push(event);
        }
        if old.disable_while_typing != new.disable_while_typing {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::DisableWhileTyping(
                new.disable_while_typing,
            )));
            events.push(event);
        }
        if old.left_handed != new.left_handed {
            let event = Event::Input(InputEvent::Mouse(MouseEvent::LeftHanded(new.left_handed)));
            events.push(event);
        }
        if old.middle_button_emulation != new.middle_button_emulation {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::MiddleButtonEmulation(
                new.middle_button_emulation,
            )));
            events.push(event);
        }
        if old.rotation_angle != new.rotation_angle {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::RotationAngle(
                new.rotation_angle,
            )));
            events.push(event);
        }
        if old.scroll_config != new.scroll_config {
            let event = Event::Input(InputEvent::Mouse(MouseEvent::ScrollConfig(
                new.scroll_config.clone(),
            )));

            // TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
            // if equivalent fine-grained events are present.
            events.push(event);

            if let (Some(old_scroll), Some(new_scroll)) =
                (old.scroll_config, new.scroll_config.clone())
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
        if old.tap_config != new.tap_config {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::TapConfig(
                new.tap_config.clone(),
            )));

            // TODO: Redundant when all sub-field events are emitted. IPC handlers should ignore this
            // if equivalent fine-grained events are present.
            events.push(event);
        }
        if old.map_to_output != new.map_to_output {
            // Unreachable: cosmic-settings currently does not produce this event
            let event = Event::Input(InputEvent::Mouse(MouseEvent::MapToOutput(
                new.map_to_output,
            )));
            events.push(event);
        }

        events
    }
}

impl KeyboardEvent {
    // #todo: convert it to a &self methods pub fn from(&self, new: XkbConfig) -> Vec<Event> where &self is the old config
    // I am unable to decide good name so leaving it :)
    pub fn from(old: XkbConfig, new: XkbConfig) -> Vec<Event> {
        if old == new {
            return vec![];
        }

        let mut events = Vec::new();

        if old.rules != new.rules {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Rules(
                new.rules.clone(),
            )));
            events.push(event);
        }
        if old.model != new.model {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Model(
                new.model.clone(),
            )));
            events.push(event);
        }
        if old.layout != new.layout {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Layout(
                new.layout.clone(),
            )));
            events.push(event);
        }
        if old.variant != new.variant {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Variant(
                new.variant.clone(),
            )));
            events.push(event);
        }
        if old.options != new.options {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::Options(
                new.options.clone(),
            )));
            events.push(event);
        }
        if old.repeat_delay != new.repeat_delay {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::RepeatDelay(
                new.repeat_delay,
            )));
            events.push(event);
        }
        if old.repeat_rate != new.repeat_rate {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::RepeatRate(
                new.repeat_rate,
            )));
            events.push(event);
        }

        events
    }
}

impl KeyboardEvent {
    pub fn from_keyboard_config(old: KeyboardConfig, new: KeyboardConfig) -> Vec<Event> {
        if old == new {
            return vec![];
        }

        let mut events = Vec::new();

        if old.numlock_state != new.numlock_state {
            let event = Event::Input(InputEvent::Keyboard(KeyboardEvent::NumLock(
                new.numlock_state,
            )));
            events.push(event);
        }

        events
    }
}