use std::error::Error;

use crate::event::input::{KeyboardEvent, MouseEvent, TouchpadEvent};

use cosmic_comp_config::input::{
    AccelConfig, ClickMethod, DeviceState, ScrollConfig, ScrollMethod, TapButtonMap, TapConfig,
};

use cosmic_comp_config::NumlockState;

pub type InputResult = Result<(), Box<dyn Error + Send + Sync>>;

/// Compositor input interface. Implement this for each compositor backend.
pub trait Input {
    fn apply_keyboard_event(&self, event: KeyboardEvent) -> InputResult {
        match event {
            KeyboardEvent::Rules(v) => self.keyboard_rules(v),
            KeyboardEvent::Model(v) => self.keyboard_model(v),
            KeyboardEvent::Layout(v) => self.keyboard_layout(v),
            KeyboardEvent::Variant(v) => self.keyboard_variant(v),
            KeyboardEvent::Options(v) => self.keyboard_options(v),
            KeyboardEvent::RepeatDelay(v) => self.keyboard_repeat_delay(v),
            KeyboardEvent::RepeatRate(v) => self.keyboard_repeat_rate(v),
            KeyboardEvent::NumLock(v) => self.numslock_state(v),
        }
    }

    fn apply_touchpad_event(&self, event: TouchpadEvent) -> InputResult {
        match event {
            TouchpadEvent::State(v) => self.touchpad_state(v),
            TouchpadEvent::Acceleration(v) => self.touchpad_acceleration(v),
            TouchpadEvent::Calibration(v) => self.touchpad_calibration(v),
            TouchpadEvent::ClickMethod(v) => self.touchpad_click_method(v),
            TouchpadEvent::DisableWhileTyping(v) => self.touchpad_disable_while_typing(v),
            TouchpadEvent::LeftHanded(v) => self.touchpad_left_handed(v),
            TouchpadEvent::MiddleButtonEmulation(v) => self.touchpad_middle_button_emulation(v),
            TouchpadEvent::RotationAngle(v) => self.touchpad_rotation_angle(v),
            TouchpadEvent::ScrollConfig(v) => self.touchpad_scroll_config(v),
            TouchpadEvent::ScrollMethod(v) => self.touchpad_scroll_method(v),
            TouchpadEvent::NaturalScroll(v) => self.touchpad_natural_scroll(v),
            TouchpadEvent::ScrollFactor(v) => self.touchpad_scroll_factor(v),
            TouchpadEvent::ScrollButton(v) => self.touchpad_scroll_button(v),
            TouchpadEvent::TapConfig(v) => self.touchpad_tap_config(v),
            TouchpadEvent::TapEnabled(v) => self.touchpad_tap_enabled(v),
            TouchpadEvent::TapButtonMap(v) => self.touchpad_tap_button_map(v),
            TouchpadEvent::TapDrag(v) => self.touchpad_tap_drag(v),
            TouchpadEvent::TapDragLock(v) => self.touchpad_tap_drag_lock(v),
            TouchpadEvent::MapToOutput(v) => self.touchpad_map_to_output(v),
        }
    }

    fn apply_mouse_event(&self, event: MouseEvent) -> InputResult {
        match event {
            MouseEvent::State(v) => self.mouse_state(v),
            MouseEvent::Acceleration(v) => self.mouse_acceleration(v),
            MouseEvent::Calibration(v) => self.mouse_calibration(v),
            MouseEvent::ClickMethod(v) => self.mouse_click_method(v),
            MouseEvent::DisableWhileTyping(v) => self.mouse_disable_while_typing(v),
            MouseEvent::LeftHanded(v) => self.mouse_left_handed(v),
            MouseEvent::MiddleButtonEmulation(v) => self.mouse_middle_button_emulation(v),
            MouseEvent::RotationAngle(v) => self.mouse_rotation_angle(v),
            MouseEvent::ScrollConfig(v) => self.mouse_scroll_config(v),
            MouseEvent::ScrollMethod(v) => self.mouse_scroll_method(v),
            MouseEvent::NaturalScroll(v) => self.mouse_natural_scroll(v),
            MouseEvent::ScrollFactor(v) => self.mouse_scroll_factor(v),
            MouseEvent::ScrollButton(v) => self.mouse_scroll_button(v),
            MouseEvent::TapConfig(v) => self.mouse_tap_config(v),
            MouseEvent::MapToOutput(v) => self.mouse_map_to_output(v),
        }
    }

    fn keyboard_rules(&self, rules: String) -> InputResult {
        eprintln!("keyboard_rules not implemented: {:?}", rules);
        Ok(())
    }
    fn keyboard_model(&self, model: String) -> InputResult {
        eprintln!("keyboard_model not implemented: {:?}", model);
        Ok(())
    }
    fn keyboard_layout(&self, layout: String) -> InputResult {
        eprintln!("keyboard_layout not implemented: {:?}", layout);
        Ok(())
    }
    fn keyboard_variant(&self, variant: String) -> InputResult {
        eprintln!("keyboard_variant not implemented: {:?}", variant);
        Ok(())
    }
    fn keyboard_options(&self, options: Option<String>) -> InputResult {
        eprintln!("keyboard_options not implemented: {:?}", options);
        Ok(())
    }
    fn keyboard_repeat_delay(&self, delay: u32) -> InputResult {
        eprintln!("keyboard_repeat_delay not implemented: {:?}", delay);
        Ok(())
    }
    fn keyboard_repeat_rate(&self, rate: u32) -> InputResult {
        eprintln!("keyboard_repeat_rate not implemented: {:?}", rate);
        Ok(())
    }

    fn numslock_state(&self, state: NumlockState) -> InputResult {
        eprintln!("numslock_state not implemented: {:?}", state);
        Ok(())
    }

    fn touchpad_state(&self, state: DeviceState) -> InputResult {
        eprintln!("touchpad_state not implemented: {:?}", state);
        Ok(())
    }
    fn touchpad_acceleration(&self, accel: Option<AccelConfig>) -> InputResult {
        eprintln!("touchpad_acceleration not implemented: {:?}", accel);
        Ok(())
    }
    fn touchpad_calibration(&self, cal: Option<[f32; 6]>) -> InputResult {
        eprintln!("touchpad_calibration not implemented: {:?}", cal);
        Ok(())
    }
    fn touchpad_click_method(&self, method: Option<ClickMethod>) -> InputResult {
        eprintln!("touchpad_click_method not implemented: {:?}", method);
        Ok(())
    }
    fn touchpad_disable_while_typing(&self, enabled: Option<bool>) -> InputResult {
        eprintln!(
            "touchpad_disable_while_typing not implemented: {:?}",
            enabled
        );
        Ok(())
    }
    fn touchpad_left_handed(&self, enabled: Option<bool>) -> InputResult {
        eprintln!("touchpad_left_handed not implemented: {:?}", enabled);
        Ok(())
    }
    fn touchpad_middle_button_emulation(&self, enabled: Option<bool>) -> InputResult {
        eprintln!(
            "touchpad_middle_button_emulation not implemented: {:?}",
            enabled
        );
        Ok(())
    }
    fn touchpad_rotation_angle(&self, angle: Option<u32>) -> InputResult {
        eprintln!("touchpad_rotation_angle not implemented: {:?}", angle);
        Ok(())
    }
    fn touchpad_scroll_config(&self, config: Option<ScrollConfig>) -> InputResult {
        eprintln!("touchpad_scroll_config not implemented: {:?}", config);
        Ok(())
    }
    fn touchpad_scroll_method(&self, method: Option<ScrollMethod>) -> InputResult {
        eprintln!("touchpad_scroll_method not implemented: {:?}", method);
        Ok(())
    }
    fn touchpad_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        eprintln!("touchpad_natural_scroll not implemented: {:?}", enabled);
        Ok(())
    }
    fn touchpad_scroll_factor(&self, factor: Option<f64>) -> InputResult {
        eprintln!("touchpad_scroll_factor not implemented: {:?}", factor);
        Ok(())
    }
    fn touchpad_scroll_button(&self, button: Option<u32>) -> InputResult {
        eprintln!("touchpad_scroll_button not implemented: {:?}", button);
        Ok(())
    }
    fn touchpad_tap_config(&self, config: Option<TapConfig>) -> InputResult {
        eprintln!("touchpad_tap_config not implemented: {:?}", config);
        Ok(())
    }
    fn touchpad_tap_enabled(&self, enabled: bool) -> InputResult {
        eprintln!("touchpad_tap_enabled not implemented: {:?}", enabled);
        Ok(())
    }
    fn touchpad_tap_button_map(&self, map: Option<TapButtonMap>) -> InputResult {
        eprintln!("touchpad_tap_button_map not implemented: {:?}", map);
        Ok(())
    }
    fn touchpad_tap_drag(&self, enabled: bool) -> InputResult {
        eprintln!("touchpad_tap_drag not implemented: {:?}", enabled);
        Ok(())
    }
    fn touchpad_tap_drag_lock(&self, enabled: bool) -> InputResult {
        eprintln!("touchpad_tap_drag_lock not implemented: {:?}", enabled);
        Ok(())
    }
    fn touchpad_map_to_output(&self, output: Option<String>) -> InputResult {
        eprintln!("touchpad_map_to_output not implemented: {:?}", output);
        Ok(())
    }

    fn mouse_state(&self, state: DeviceState) -> InputResult {
        eprintln!("mouse_state not implemented: {:?}", state);
        Ok(())
    }
    fn mouse_acceleration(&self, accel: Option<AccelConfig>) -> InputResult {
        eprintln!("mouse_acceleration not implemented: {:?}", accel);
        Ok(())
    }
    fn mouse_calibration(&self, cal: Option<[f32; 6]>) -> InputResult {
        eprintln!("mouse_calibration not implemented: {:?}", cal);
        Ok(())
    }
    fn mouse_click_method(&self, method: Option<ClickMethod>) -> InputResult {
        eprintln!("mouse_click_method not implemented: {:?}", method);
        Ok(())
    }
    fn mouse_disable_while_typing(&self, enabled: Option<bool>) -> InputResult {
        eprintln!("mouse_disable_while_typing not implemented: {:?}", enabled);
        Ok(())
    }
    fn mouse_left_handed(&self, enabled: Option<bool>) -> InputResult {
        eprintln!("mouse_left_handed not implemented: {:?}", enabled);
        Ok(())
    }
    fn mouse_middle_button_emulation(&self, enabled: Option<bool>) -> InputResult {
        eprintln!(
            "mouse_middle_button_emulation not implemented: {:?}",
            enabled
        );
        Ok(())
    }
    fn mouse_rotation_angle(&self, angle: Option<u32>) -> InputResult {
        eprintln!("mouse_rotation_angle not implemented: {:?}", angle);
        Ok(())
    }
    fn mouse_scroll_config(&self, config: Option<ScrollConfig>) -> InputResult {
        eprintln!("mouse_scroll_config not implemented: {:?}", config);
        Ok(())
    }
    fn mouse_scroll_method(&self, method: Option<ScrollMethod>) -> InputResult {
        eprintln!("mouse_scroll_method not implemented: {:?}", method);
        Ok(())
    }
    fn mouse_natural_scroll(&self, enabled: Option<bool>) -> InputResult {
        eprintln!("mouse_natural_scroll not implemented: {:?}", enabled);
        Ok(())
    }
    fn mouse_scroll_factor(&self, factor: Option<f64>) -> InputResult {
        eprintln!("mouse_scroll_factor not implemented: {:?}", factor);
        Ok(())
    }
    fn mouse_scroll_button(&self, button: Option<u32>) -> InputResult {
        eprintln!("mouse_scroll_button not implemented: {:?}", button);
        Ok(())
    }
    fn mouse_tap_config(&self, config: Option<TapConfig>) -> InputResult {
        eprintln!("mouse_tap_config not implemented: {:?}", config);
        Ok(())
    }
    fn mouse_map_to_output(&self, output: Option<String>) -> InputResult {
        eprintln!("mouse_map_to_output not implemented: {:?}", output);
        Ok(())
    }
}
