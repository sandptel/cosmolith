use cosmolith::compositor::input::InputResult;
use niri_ipc::{socket::Socket, Action, Request};

fn send_action(action: Action) -> InputResult {
    let mut socket = Socket::connect()?;

    let response = socket.send(Request::Action(action))?;

    match response {
        Ok(_) => Ok(()),
        Err(err_msg) => Err(Box::<dyn std::error::Error + Send + Sync>::from(err_msg)),
    }
}

fn main() -> hyprland::Result<()> {
    // 1. Change layout to index 0 of niri config ( to change to a particular layout, you need to have it in niri config )
    match send_action(Action::SwitchLayout {
        layout: niri_ipc::LayoutSwitchTarget::Index(0),
    }) {
        Ok(_) => {
            println!("Keyboard layout changed to the first layout in the list in niri config!");
        }
        Err(err) => {
            println!("{}", err);
        }
    }

    Ok(())
}
