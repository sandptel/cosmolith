pub mod hyprland;
pub mod input;
pub mod kde;
pub mod sway;
use crate::event::Event;
use std::error::Error;
pub type CompositorResult = Result<(), Box<dyn Error + Send + Sync>>;
/// Central compositor interface used by the dispatcher.
#[allow(unused)]
pub trait Compositor {
    /// Initialize compositor integration (set up IPC, validate availability).
    fn init(&mut self) -> CompositorResult;

    /// Human-readable compositor name.
    fn name(&self) -> &'static str;

    /// Fast check to see if the compositor is running/available.
    fn is_running(&self) -> bool;

    /// Whether a given event is supported by this compositor.
    fn supports(&self, event: &Event) -> bool;

    /// Apply a single event to the compositor.
    fn apply_event(&self, event: Event) -> CompositorResult;

    /// Optional reload hook if compositor exposes a reload action.
    fn reload(&self) -> CompositorResult;

    /// Optional shutdown/cleanup hook.
    fn shutdown(&self) -> CompositorResult;
}

pub fn init_compositor(desktop: crate::identifier::Desktop) -> Option<Box<dyn Compositor>> {
    match desktop {
        crate::identifier::Desktop::Hyprland => {
            let mut compositor = hyprland::Hyprland::new();
            if compositor.init().is_ok() {
                return Some(Box::new(compositor));
            }
            None
        }
        crate::identifier::Desktop::Sway => {
            let mut compositor = sway::Sway::new();
            if compositor.init().is_ok() {
                return Some(Box::new(compositor));
            }
            None
        }
        crate::identifier::Desktop::Kde => {
            let mut compositor = kde::Kde::new();
            if compositor.init().is_ok() {
                return Some(Box::new(compositor));
            }
            None
        }
        _ => None,
    }
}
