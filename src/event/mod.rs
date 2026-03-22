pub mod input;
pub use input::InputEvent;

pub mod shortcuts;
pub use shortcuts::ShortcutEvent;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum Event {
    Input(InputEvent),
    Shortcut(ShortcutEvent),
}

// impl InputEvent {
//     pub fn from(old: &InputConfig, new: &InputConfig) -> Vec<InputEvent> {
//         // This will convert the config to events and then send to whereever its is required accordingly.
//     }
// }
