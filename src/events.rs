use std::time::Instant;
use std::{sync::mpsc, thread, time::Duration};

use crossterm::event::{self as CrossTermEvent, Event as CEvent, KeyCode};

pub enum EventType<I> {
    Input(I),
    Tick,
}

pub struct Events {
    rx: mpsc::Receiver<EventType<KeyCode>>,
}

impl Events {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel();
        let tick_rate = Duration::from_millis(100);

        thread::spawn(move || {
            let mut last_tick = Instant::now();

            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or_else(|| Duration::from_secs(0));

                if CrossTermEvent::poll(timeout).expect("Unable to poll crossterm event") {
                    if let CEvent::Key(key) =
                        CrossTermEvent::read().expect("Unable to read crossterm event")
                    {
                        tx.send(EventType::Input(key.code))
                            .expect("Unable to send key event");
                    }
                }

                if last_tick.elapsed() >= tick_rate && tx.send(EventType::Tick).is_ok() {
                    last_tick = Instant::now();
                }
            }
        });

        Events { rx }
    }

    pub fn next(&self) -> EventType<KeyCode> {
        self.rx.recv().expect("Unable to recieve events")
    }
}
