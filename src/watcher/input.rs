// Watch Input Config Changes

use std::{error::Error, sync::mpsc::Sender};

use cosmic_comp_config::input::{
    AccelConfig, AccelProfile, ClickMethod, DeviceState, InputConfig, ScrollConfig, ScrollMethod,
    TapButtonMap, TapConfig,
};
use cosmic_config::{Config, ConfigGet};

use crate::event::Event;
use std::sync::{Arc, Mutex};

pub struct InputState {
    touchpad: Option<InputConfig>,
    mouse: Option<InputConfig>,
    // Will be added later after identifying its type
    // keyboard:
}

#[derive(Clone, Debug, PartialEq)]
pub enum InputEvent {
    TouchPad(TouchpadEvent),
    Mouse(MouseEvent),
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

pub fn start_input_watcher(
    tx: &Arc<Mutex<Sender<Event>>>,
) -> Result<Box<dyn std::any::Any + Send>, Box<dyn Error>> {
    let config = Config::new("com.system76.CosmicComp", 1)?;
    let state = Arc::new(Mutex::new(InputState {
        touchpad: config.get::<InputConfig>("input_touchpad").ok(),
        mouse: config.get::<InputConfig>("input_default").ok(),
    }));

    // Keep the watcher alive for the lifetime of the program.
    let watcher = config.watch({
        let tx = Arc::clone(&tx);
        let state = Arc::clone(&state);
        move |cfg: &Config, keys| {
            if let Ok(sender) = tx.lock() {
                if let Ok(mut state) = state.lock() {
                    let events = state.from(cfg, keys);
                    for event in events {
                        if let Err(err) = sender.send(event) {
                            eprintln!("Failed to send input event: {err}");
                        }
                    }
                }
            }
        }
    })?;

    Ok(Box::new(watcher))
}

impl InputState {
    pub fn from(&mut self, cfg: &Config, keys: &[String]) -> Vec<Event> {
        let mut events = Vec::new();
        for key in keys {
            match key.as_str() {
                "input_touchpad" => match cfg.get::<InputConfig>(key) {
                    Ok(new_config) => {
                        if let Some(old) = self.touchpad.clone() {
                            events.extend(from_touchpad(old, new_config.clone()));
                        }
                        self.touchpad = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
                "input_default" => match cfg.get::<InputConfig>(key) {
                    Ok(new_config) => {
                        if let Some(old) = self.mouse.clone() {
                            events.extend(from_mouse(old, new_config.clone()));
                        }
                        self.mouse = Some(new_config);
                    }
                    Err(e) => {
                        eprintln!("Failed to get changed config due to the error: {:?}", e);
                    }
                },
                x => {
                    eprintln!(
                        "Unknown key found in Input (com.system76.CosmicComp): {}",
                        x
                    );
                }
            }
        }
        events
    }
}

pub fn from_touchpad(old: InputConfig, new: InputConfig) -> Vec<Event> {
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

        if let (Some(old_scroll), Some(new_scroll)) = (old.scroll_config, new.scroll_config.clone())
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

pub fn from_mouse(old: InputConfig, new: InputConfig) -> Vec<Event> {
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

        if let (Some(old_scroll), Some(new_scroll)) = (old.scroll_config, new.scroll_config.clone())
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
