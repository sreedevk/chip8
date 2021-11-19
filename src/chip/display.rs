use std::io;
use tui::Terminal;
use tui::backend::TermionBackend;
use termion::raw::{IntoRawMode, RawTerminal};
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction};

pub struct Manager {
    terminal: Terminal<TermionBackend<RawTerminal<io::Stdout>>>
}

impl std::fmt::Debug for Manager {
    fn fmt(&self, _formatter: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        return Ok(())
    }
}

impl Manager {
    pub fn new() -> Manager {
        let stdout       = io::stdout().into_raw_mode().unwrap();
        let backend      = TermionBackend::new(stdout);
        let terminal     = Terminal::new(backend).unwrap();
        Manager { terminal }
    }

    pub fn render(&mut self) {
        self.terminal.draw(|f| {
            let block = Block::default()
                .title("Chip8")
                .borders(Borders::ALL);

            f.render_widget(block, f.size());
        }).unwrap();
    }
}
