use cosmic_settings_config::shortcuts::{Action, Binding};

#[derive(Debug, Clone)]
pub enum ShortcutEvent {
    Add { binding: Binding, action: Action },
    Remove { binding: Binding },
}
