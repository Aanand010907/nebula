use std::time::Duration;

use crossterm::event::{self, Event, KeyEvent, KeyEventKind};
use tokio::sync::mpsc;

/// Application events — either terminal events or internal ticks.
#[derive(Debug)]
pub enum AppEvent {
    /// A key was pressed.
    Key(KeyEvent),
    /// Periodic tick for animations and async result polling.
    Tick,
    /// Terminal was resized.
    Resize(u16, u16),
}

/// Event handler that polls crossterm events and emits AppEvents.
/// Runs on a dedicated tokio task to avoid blocking the main thread.
pub struct EventHandler {
    rx: mpsc::UnboundedReceiver<AppEvent>,
    // Keep the handle alive so the task isn't dropped
    _task: tokio::task::JoinHandle<()>,
}

impl EventHandler {
    /// Create a new event handler with the given tick rate in milliseconds.
    pub fn new(tick_rate_ms: u64) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        let tick_rate = Duration::from_millis(tick_rate_ms);

        let task = tokio::spawn(async move {
            loop {
                // Poll crossterm for events with timeout
                let has_event = tokio::task::block_in_place(|| {
                    event::poll(tick_rate).unwrap_or(false)
                });

                if has_event {
                    let event = tokio::task::block_in_place(|| {
                        event::read()
                    });

                    match event {
                        Ok(Event::Key(key)) => {
                            // Only handle key press events, not release/repeat
                            if key.kind == KeyEventKind::Press {
                                if tx.send(AppEvent::Key(key)).is_err() {
                                    return; // Channel closed, app is shutting down
                                }
                            }
                        }
                        Ok(Event::Resize(w, h)) => {
                            let _ = tx.send(AppEvent::Resize(w, h));
                        }
                        _ => {}
                    }
                } else {
                    // Timeout — emit a tick
                    if tx.send(AppEvent::Tick).is_err() {
                        return;
                    }
                }
            }
        });

        Self { rx, _task: task }
    }

    /// Wait for the next event.
    pub async fn next(&mut self) -> anyhow::Result<AppEvent> {
        self.rx
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("Event channel closed"))
    }
}
