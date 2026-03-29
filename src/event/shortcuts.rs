use cosmic_settings_config::shortcuts::{
    Action as CosmicAction, Binding,
    action::{Direction as CosmicDirection, FocusDirection as CosmicFocusDirection, System as CosmicSystem},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FocusDirection { Left, Right, Up, Down }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Direction { Left, Right, Up, Down }

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemAction {
    Launcher,
    AppLibrary,
    Terminal,
    WebBrowser,
    HomeFolder,
    Screenshot,
    BrightnessDown,
    BrightnessUp,
    VolumeLower,
    VolumeRaise,
    Mute,
    MuteMic,
    PlayPause,
    PlayNext,
    PlayPrev,
    LockScreen,
    LogOut,
    PowerOff,
    Suspend,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Shortcut {
    Close,
    Focus(FocusDirection),
    Move(Direction),
    Workspace(String),
    MoveToWorkspace(String),
    Terminate,
    Custom(String),
    System(SystemAction),
    Unknown,
}

impl From<CosmicAction> for Shortcut {
    fn from(action: CosmicAction) -> Self {
        match action {
            CosmicAction::Close => Shortcut::Close,
            CosmicAction::Focus(CosmicFocusDirection::Left) => Shortcut::Focus(FocusDirection::Left),
            CosmicAction::Focus(CosmicFocusDirection::Right) => Shortcut::Focus(FocusDirection::Right),
            CosmicAction::Focus(CosmicFocusDirection::Up) => Shortcut::Focus(FocusDirection::Up),
            CosmicAction::Focus(CosmicFocusDirection::Down) => Shortcut::Focus(FocusDirection::Down),
            CosmicAction::Move(CosmicDirection::Left) => Shortcut::Move(Direction::Left),
            CosmicAction::Move(CosmicDirection::Right) => Shortcut::Move(Direction::Right),
            CosmicAction::Move(CosmicDirection::Up) => Shortcut::Move(Direction::Up),
            CosmicAction::Move(CosmicDirection::Down) => Shortcut::Move(Direction::Down),
            CosmicAction::Workspace(id) => Shortcut::Workspace(id.to_string()),
            CosmicAction::MoveToWorkspace(id) => Shortcut::MoveToWorkspace(id.to_string()),
            CosmicAction::Terminate => Shortcut::Terminate,
            CosmicAction::Spawn(cmd) => Shortcut::Custom(cmd),
            CosmicAction::System(sys) => {
                let sys_action = match sys {
                    CosmicSystem::Launcher => SystemAction::Launcher,
                    CosmicSystem::AppLibrary => SystemAction::AppLibrary,
                    CosmicSystem::Terminal => SystemAction::Terminal,
                    CosmicSystem::WebBrowser => SystemAction::WebBrowser,
                    CosmicSystem::HomeFolder => SystemAction::HomeFolder,
                    CosmicSystem::Screenshot => SystemAction::Screenshot,
                    CosmicSystem::BrightnessDown => SystemAction::BrightnessDown,
                    CosmicSystem::BrightnessUp => SystemAction::BrightnessUp,
                    CosmicSystem::VolumeLower => SystemAction::VolumeLower,
                    CosmicSystem::VolumeRaise => SystemAction::VolumeRaise,
                    CosmicSystem::Mute => SystemAction::Mute,
                    CosmicSystem::MuteMic => SystemAction::MuteMic,
                    CosmicSystem::PlayPause => SystemAction::PlayPause,
                    CosmicSystem::PlayNext => SystemAction::PlayNext,
                    CosmicSystem::PlayPrev => SystemAction::PlayPrev,
                    CosmicSystem::LockScreen => SystemAction::LockScreen,
                    CosmicSystem::LogOut => SystemAction::LogOut,
                    CosmicSystem::PowerOff => SystemAction::PowerOff,
                    CosmicSystem::Suspend => SystemAction::Suspend,
                    _ => SystemAction::Unknown,
                };
                Shortcut::System(sys_action)
            }
            _ => Shortcut::Unknown,
        }
    }
}

#[derive(Debug, Clone)]
pub enum ShortcutEvent {
    Add { shortcut: Shortcut, binding: Binding },
    Remove { shortcut: Shortcut, binding: Binding },
}
