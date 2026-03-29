use crate::compositor::CompositorResult;
use crate::event::shortcuts::{Shortcut as ShortcutAction, ShortcutEvent};
use cosmic_settings_config::shortcuts::Binding;

/// Compositor shortcut interface. Implement this for each compositor backend.
pub trait Shortcut {
    fn apply_shortcut_event(&self, event: ShortcutEvent) -> CompositorResult {
        match event {
            ShortcutEvent::Add { shortcut, binding } => self.add_shortcut(shortcut, binding),
            ShortcutEvent::Remove { shortcut, binding } => self.remove_shortcut(shortcut, binding),
        }
    }

    fn add_shortcut(&self, _shortcut: ShortcutAction, _binding: Binding) -> CompositorResult {
        eprintln!("add_shortcut not implemented");
        Ok(())
    }

    fn remove_shortcut(&self, _shortcut: ShortcutAction, _binding: Binding) -> CompositorResult {
        eprintln!("remove_shortcut not implemented");
        Ok(())
    }
}
