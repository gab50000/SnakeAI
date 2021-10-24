use std::io;
use std::sync::mpsc;
use std::thread;
use termion::event::{Event, Key, MouseEvent};
use termion::input::{Events, TermRead};

type MaybeKey = Option<termion::event::Key>;
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Quit,
}

pub struct InputHandler {
    rx: mpsc::Receiver<Option<termion::event::Key>>,
}

impl InputHandler {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::sync_channel(1);
        let obj = Self { rx };
        thread::spawn(move || Self::wait_for_input(tx));
        obj
    }

    pub fn next(&self) -> Option<termion::event::Key> {
        let key = self.rx.try_recv();
        match key {
            Ok(x) => x,
            _ => None,
        }
    }

    fn wait_for_input(tx: mpsc::SyncSender<MaybeKey>) -> Result<(), mpsc::SendError<MaybeKey>> {
        let stdin = io::stdin();
        let events = stdin.events();
        for event in events {
            match event {
                Ok(Event::Key(Key::Char('q'))) => {
                    tx.send(Some(Key::Char('q')))?;
                    break;
                }
                Ok(Event::Key(key)) => tx.send(Some(key))?,
                _ => break,
            };
        }
        Ok(())
    }

    pub fn determine_action(&self) -> Option<Action> {
        match self.next() {
            Some(c) => match c {
                Key::Char('q') => Some(Action::Quit),
                Key::Left => Some(Action::Left),
                Key::Right => Some(Action::Right),
                Key::Up => Some(Action::Up),
                Key::Down => Some(Action::Down),
                _ => None,
            },
            _ => None,
        }
    }
}
