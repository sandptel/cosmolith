use std::env;
use std::sync::Mutex;

use crate::compositor::input::{Input, InputResult};
use crate::compositor::{Compositor, CompositorResult};
use crate::event::input::InputEvent;
use crate::event::Event;

use cosmic_comp_config::input::{
    AccelConfig, ClickMethod, ScrollConfig, ScrollMethod, TapButtonMap, TapConfig,
};

use niri_ipc::socket::Socket;
use niri_ipc::{Action, Request, Response};

pub struct Niri {
    socket: Mutex<Option<Socket>>,
}

impl Niri {
    pub fn new() -> Self {
        Self {
            socket: Mutex::new(None),
        }
    }

    fn send_action(&self, action: Action) -> InputResult {
        let mut guard = self.socket.lock().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Niri connection lock poisoned")
        })?;

        if guard.is_none() {
            *guard = Some(Socket::connect()?);
        }

        let socket = guard.as_mut().unwrap();

        let response = socket.send(Request::Action(action))?;

        match response {
            Ok(_) => Ok(()),
            Err(err_msg) => Err(Box::<dyn std::error::Error + Send + Sync>::from(err_msg)),
        }
    }

    fn request_socket(
        &self,
        req: Request,
    ) -> Result<Response, Box<dyn std::error::Error + Send + Sync>> {
        let mut guard = self.socket.lock().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Niri connection lock poisoned")
        })?;

        if guard.is_none() {
            *guard = Some(Socket::connect()?);
        }

        let socket = guard.as_mut().unwrap();

        let response = socket.send(req)?;

        match response {
            Ok(res) => Ok(res),
            Err(err_msg) => Err(Box::<dyn std::error::Error + Send + Sync>::from(err_msg)),
        }
    }

    // fn set_bool_action(
    //     &self,
    //     value: Option<bool>,
    //     on_true: Action,
    //     on_false: Action,
    // ) -> InputResult {
    //     match value {
    //         Some(true) => self.send_action(on_true),
    //         Some(false) => self.send_action(on_false),
    //         None => Ok(()),
    //     }
    // }
}

impl Compositor for Niri {
    fn init(&mut self) -> CompositorResult {
        let mut guard = self.socket.lock().map_err(|_| {
            std::io::Error::new(std::io::ErrorKind::Other, "Niri connection lock poisoned")
        })?;
        *guard = Some(Socket::connect()?);
        Ok(())
    }

    fn name(&self) -> &'static str {
        "Niri"
    }

    fn is_running(&self) -> bool {
        env::var("NIRI_SOCKET").is_ok()
    }

    fn supports(&self, event: &Event) -> bool {
        matches!(event, Event::Input(_))
    }

    fn apply_event(&self, event: Event) -> CompositorResult {
        match event {
            Event::Input(InputEvent::Keyboard(ev)) => self.apply_keyboard_event(ev)?,
            Event::Input(InputEvent::Mouse(ev)) => self.apply_mouse_event(ev)?,
            Event::Input(InputEvent::TouchPad(ev)) => self.apply_touchpad_event(ev)?,
        }
        Ok(())
    }

    fn reload(&self) -> CompositorResult {
        Ok(())
    }

    fn shutdown(&self) -> CompositorResult {
        Ok(())
    }
}

impl Input for Niri {
    /* Keyboard */

    fn keyboard_layout(&self, _layout: String) -> InputResult {
        // let keyboard_layouts = self.request_socket(Request::KeyboardLayouts)?;
        //
        // if let Response::KeyboardLayouts(layouts) = keyboard_layouts {
        //     println!("Keyboard layouts: {:?}", layouts);
        //
        //     let index = layouts
        //         .names
        //         .iter()
        //         .position(|name| name.to_lowercase().contains(&layout.to_lowercase()));
        //
        //     if let Some(idx) = index {
        //         return self.send_action(Action::SwitchLayout {
        //             layout: niri_ipc::LayoutSwitchTarget::Index(idx as u8),
        //         });
        //     } else {
        //         let msg = format!("Layout '{}' not found in Niri config", layout);
        //         return Err(Box::<dyn std::error::Error + Send + Sync>::from(msg));
        //     }
        // }

        Ok(())
    }

    fn keyboard_options(&self, _options: Option<String>) -> InputResult {
        todo!()
    }

    fn keyboard_repeat_delay(&self, _delay: u32) -> InputResult {
        todo!()
    }

    fn keyboard_repeat_rate(&self, _rate: u32) -> InputResult {
        todo!()
    }

    /* Touchpad */

    fn touchpad_state(&self, _state: cosmic_comp_config::input::DeviceState) -> InputResult {
        todo!()
    }

    fn touchpad_acceleration(&self, _accel: Option<AccelConfig>) -> InputResult {
        todo!()
    }

    fn touchpad_click_method(&self, _method: Option<ClickMethod>) -> InputResult {
        todo!()
    }

    fn touchpad_disable_while_typing(&self, _enabled: Option<bool>) -> InputResult {
        todo!()
    }

    fn touchpad_left_handed(&self, _enabled: Option<bool>) -> InputResult {
        todo!()
    }
    fn touchpad_middle_button_emulation(&self, _enabled: Option<bool>) -> InputResult {
        todo!()
    }

    fn touchpad_rotation_angle(&self, _angle: Option<u32>) -> InputResult {
        todo!()
    }

    fn touchpad_scroll_config(&self, _config: Option<ScrollConfig>) -> InputResult {
        todo!()
    }

    fn touchpad_scroll_method(&self, _method: Option<ScrollMethod>) -> InputResult {
        todo!()
    }

    fn touchpad_natural_scroll(&self, _enabled: Option<bool>) -> InputResult {
        todo!()
    }

    fn touchpad_scroll_factor(&self, _factor: Option<f64>) -> InputResult {
        todo!()
    }

    fn touchpad_scroll_button(&self, _button: Option<u32>) -> InputResult {
        todo!()
    }

    fn touchpad_tap_config(&self, _config: Option<TapConfig>) -> InputResult {
        todo!()
    }

    fn touchpad_tap_enabled(&self, _enabled: bool) -> InputResult {
        todo!()
    }

    fn touchpad_tap_button_map(&self, _map: Option<TapButtonMap>) -> InputResult {
        todo!()
    }

    fn touchpad_tap_drag(&self, _enabled: bool) -> InputResult {
        todo!()
    }

    fn touchpad_tap_drag_lock(&self, _enabled: bool) -> InputResult {
        todo!()
    }

    /* Mouse */

    fn mouse_state(&self, _state: cosmic_comp_config::input::DeviceState) -> InputResult {
        todo!()
    }

    fn mouse_acceleration(&self, _accel: Option<AccelConfig>) -> InputResult {
        todo!()
    }

    fn mouse_click_method(&self, _method: Option<ClickMethod>) -> InputResult {
        todo!()
    }

    fn mouse_disable_while_typing(&self, _enabled: Option<bool>) -> InputResult {
        todo!()
    }

    fn mouse_left_handed(&self, _enabled: Option<bool>) -> InputResult {
        todo!()
    }

    fn mouse_middle_button_emulation(&self, _enabled: Option<bool>) -> InputResult {
        todo!()
    }

    fn mouse_rotation_angle(&self, _angle: Option<u32>) -> InputResult {
        todo!()
    }

    fn mouse_scroll_config(&self, _config: Option<ScrollConfig>) -> InputResult {
        todo!()
    }

    fn mouse_scroll_method(&self, _method: Option<ScrollMethod>) -> InputResult {
        todo!()
    }

    fn mouse_natural_scroll(&self, _enabled: Option<bool>) -> InputResult {
        todo!()
    }

    fn mouse_scroll_factor(&self, _factor: Option<f64>) -> InputResult {
        todo!()
    }

    fn mouse_scroll_button(&self, _button: Option<u32>) -> InputResult {
        todo!()
    }

    fn mouse_tap_config(&self, _config: Option<TapConfig>) -> InputResult {
        todo!()
    }

    fn mouse_map_to_output(&self, _output: Option<String>) -> InputResult {
        todo!()
    }
}
