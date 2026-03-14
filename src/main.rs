// src/main.rs
use cosmic_config::Config;
use std::{
    error::Error,
    sync::{Arc, Mutex, mpsc},
    time::Duration,
};

mod watcher;
use watcher::input::{load_initial_input_events, start_input_watcher};
mod event;
use event::Event;

mod identifier;
use identifier::get_current_session;

mod compositor;
use compositor::{Compositor, init_compositor};

fn apply_startup_events(compositor: Option<&dyn Compositor>, startup_events: Vec<Event>) -> usize {
    if let Some(comp) = compositor {
        let mut unapplied = 0;

        for event in startup_events {
            if !comp.supports(&event) {
                unapplied += 1;
                continue;
            }

            if let Err(err) = comp.apply_event(event) {
                eprintln!("Failed to apply startup input event: {err}");
                unapplied += 1;
            }
        }

        unapplied
    } else {
        startup_events.len()
    }
}

fn apply_runtime_event(compositor: Option<&dyn Compositor>, event: Event) -> bool {
    if let Some(comp) = compositor {
        if !comp.supports(&event) {
            return false;
        }

        if let Err(err) = comp.apply_event(event) {
            eprintln!("Failed to apply event: {err}");
        }
    }

    true
}

fn startup_events_or_empty(result: Result<Vec<Event>, Box<dyn Error>>) -> Vec<Event> {
    match result {
        Ok(events) => events,
        Err(err) => {
            eprintln!("Failed to load startup input events: {err}");
            Vec::new()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let _config = Config::new("com.system76.CosmicComp", 1)?;
    // Channel used to receive change notifications from the watcher callback.
    let (tx, rx) = mpsc::channel::<Event>();
    let tx = Arc::new(Mutex::new(tx));

    let _watcher = start_input_watcher(&tx)?;
    let bootstrap_events = startup_events_or_empty(load_initial_input_events());

    println!("Watching for configuration changes…");

    let session = get_current_session();
    println!("You are currently running: {:?}", session);

    let compositor = init_compositor(session);
    if compositor.is_none() {
        eprintln!("No supported compositor detected. Events will be logged only.");
    }
    let unapplied_startup_events = apply_startup_events(compositor.as_deref(), bootstrap_events);
    if unapplied_startup_events > 0 {
        eprintln!(
            "Generated {unapplied_startup_events} startup input event(s) that could not be applied."
        );
    }

    loop {
        match rx.recv_timeout(Duration::from_secs(5)) {
            Ok(event) => {
                println!("Received: {:?}", event);
                let _ = apply_runtime_event(compositor.as_deref(), event);
            }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                // optional heartbeat to keep the loop responsive to Ctrl+C
                continue;
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                eprintln!("Watcher channel closed; exiting.");
                break;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compositor::{Compositor, CompositorResult};
    use crate::event::input::{InputEvent, KeyboardEvent};
    use cosmic_comp_config::NumlockState;
    use std::sync::Mutex;

    #[derive(Default)]
    struct FakeCompositor {
        events: Mutex<Vec<Event>>,
    }

    impl Compositor for FakeCompositor {
        fn init(&mut self) -> CompositorResult {
            Ok(())
        }

        fn name(&self) -> &'static str {
            "fake"
        }

        fn is_running(&self) -> bool {
            true
        }

        fn supports(&self, _event: &Event) -> bool {
            true
        }

        fn apply_event(&self, event: Event) -> CompositorResult {
            self.events.lock().unwrap().push(event);
            Ok(())
        }

        fn reload(&self) -> CompositorResult {
            Ok(())
        }

        fn shutdown(&self) -> CompositorResult {
            Ok(())
        }
    }

    fn sample_startup_events() -> Vec<Event> {
        vec![
            Event::Input(InputEvent::Keyboard(KeyboardEvent::Layout("us".into()))),
            Event::Input(InputEvent::Keyboard(KeyboardEvent::NumLock(
                NumlockState::BootOn,
            ))),
        ]
    }

    #[test]
    fn apply_startup_events_applies_all_events_when_compositor_exists() {
        let compositor = FakeCompositor::default();
        let startup_events = sample_startup_events();

        let unapplied = apply_startup_events(Some(&compositor), startup_events.clone());
        let recorded = compositor.events.lock().unwrap();

        assert_eq!(unapplied, 0);
        assert_eq!(recorded.len(), startup_events.len());
        assert!(matches!(
            recorded.first(),
            Some(Event::Input(InputEvent::Keyboard(KeyboardEvent::Layout(layout))))
                if layout == "us"
        ));
        assert!(matches!(
            recorded.get(1),
            Some(Event::Input(InputEvent::Keyboard(KeyboardEvent::NumLock(
                NumlockState::BootOn
            ))))
        ));
    }

    #[test]
    fn apply_startup_events_reports_unapplied_count_without_compositor() {
        let startup_events = sample_startup_events();

        let unapplied = apply_startup_events(None, startup_events);

        assert_eq!(unapplied, 2);
    }

    #[derive(Default)]
    struct UnsupportedCompositor;

    impl Compositor for UnsupportedCompositor {
        fn init(&mut self) -> CompositorResult {
            Ok(())
        }

        fn name(&self) -> &'static str {
            "unsupported"
        }

        fn is_running(&self) -> bool {
            true
        }

        fn supports(&self, _event: &Event) -> bool {
            false
        }

        fn apply_event(&self, _event: Event) -> CompositorResult {
            Ok(())
        }

        fn reload(&self) -> CompositorResult {
            Ok(())
        }

        fn shutdown(&self) -> CompositorResult {
            Ok(())
        }
    }

    #[test]
    fn apply_startup_events_counts_unsupported_events_as_unapplied() {
        let compositor = UnsupportedCompositor;
        let startup_events = sample_startup_events();

        let unapplied = apply_startup_events(Some(&compositor), startup_events);

        assert_eq!(unapplied, 2);
    }

    struct FailingCompositor;

    impl Compositor for FailingCompositor {
        fn init(&mut self) -> CompositorResult {
            Ok(())
        }

        fn name(&self) -> &'static str {
            "failing"
        }

        fn is_running(&self) -> bool {
            true
        }

        fn supports(&self, _event: &Event) -> bool {
            true
        }

        fn apply_event(&self, _event: Event) -> CompositorResult {
            Err(Box::new(std::io::Error::other("startup apply failed")))
        }

        fn reload(&self) -> CompositorResult {
            Ok(())
        }

        fn shutdown(&self) -> CompositorResult {
            Ok(())
        }
    }

    #[test]
    fn apply_startup_events_counts_failed_events_as_unapplied() {
        let compositor = FailingCompositor;
        let startup_events = sample_startup_events();

        let unapplied = apply_startup_events(Some(&compositor), startup_events);

        assert_eq!(unapplied, 2);
    }

    #[test]
    fn startup_events_or_empty_returns_events_on_success() {
        let events = sample_startup_events();

        let loaded = startup_events_or_empty(Ok(events.clone()));

        assert_eq!(loaded.len(), events.len());
        assert!(matches!(
            loaded.first(),
            Some(Event::Input(InputEvent::Keyboard(KeyboardEvent::Layout(layout))))
                if layout == "us"
        ));
    }

    #[test]
    fn startup_events_or_empty_degrades_to_empty_on_error() {
        let loaded = startup_events_or_empty(Err(Box::new(std::io::Error::other(
            "broken startup snapshot",
        ))));

        assert!(loaded.is_empty());
    }

    #[test]
    fn apply_runtime_event_skips_unsupported_events() {
        let compositor = UnsupportedCompositor;
        let applied = apply_runtime_event(
            Some(&compositor),
            Event::Input(InputEvent::Keyboard(KeyboardEvent::NumLock(
                NumlockState::BootOn,
            ))),
        );

        assert!(!applied);
    }
}
