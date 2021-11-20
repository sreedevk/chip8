use std::{
    io,
    sync::mpsc,
    thread,
    time::Duration
};

use termion::{
    event::Key,
    input::TermRead
};

use crate::chip::VM;

pub enum Event<T> {
    Input(T),
    Tick,
}

pub struct Manager {
    rx: mpsc::Receiver<Event<Key>>,
    input_handler: thread::JoinHandle<()>,
    tick_handler: thread::JoinHandle<()>
}

impl std::fmt::Debug for Manager {
    fn fmt(&self, _formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return Ok(())
    }
}

impl Manager {
    pub fn new() -> Manager {
        let (tx, rx) = mpsc::channel();
        let input_handler = {
            let tx = tx.clone();
            thread::spawn(move || {
                let stdin = io::stdin();
                for evt in stdin.keys() {
                    if let Ok(key) = evt {
                        if let Err(err) = tx.send(Event::Input(key)) {
                            eprintln!("{}", err);
                            return;
                        }
                    }
                }
            })
        };

        let tick_handler = {
            thread::spawn(move || {
                if let Err(err)  = tx.send(Event::Tick) {
                    eprintln!("{}", err);
                    return;
                }
                thread::sleep(Duration::from_millis(250));
            })
        };

        Manager {
            rx,
            input_handler,
            tick_handler
        }
    }

    pub fn dequeue(&self) -> Result<Event<Key>, mpsc::RecvError> {
        self.rx.recv()
    }

    pub fn handle(machine: &mut VM) -> Result<(), mpsc::RecvError> {
        if let Event::Input(key) = machine.events_man.dequeue()? {
            if key == Key::Char('q') {
                machine.running = false;
                return Ok(());
            }
        }
        Ok(())
    }
}
