use std::io;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::chip::VM;
use tui::{
    Terminal,
    backend::{Backend, TermionBackend},
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Line, Map, MapResolution, Rectangle},
    widgets::{
        Axis, BarChart, Block, Borders, Cell, Chart, Dataset, Gauge, LineGauge, List, ListItem,
        Paragraph, Row, Sparkline, Table, Tabs, Wrap, Widget,
    },
    Frame,
};

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

    pub fn render(machine: &mut VM) {
        machine.display_man.terminal.clear().unwrap();
        machine.display_man.terminal.draw(|f| {
            let main = Block::default()
                .title("Chip8")
                .borders(Borders::ALL);

            let vlayout = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints([
                    Constraint::Percentage(10),
                    Constraint::Percentage(90)
                ].as_ref())
                .split(f.size());


            let hlayout = Layout::default()
                .direction(Direction::Horizontal)
                .vertical_margin(0)
                .horizontal_margin(1)
                .constraints([
                    Constraint::Percentage(70),
                    Constraint::Percentage(30)
                ].as_ref())
                .split(vlayout[1]);

            f.render_widget(main, f.size());
            f.render_widget(Manager::generate_machine_display_block(), hlayout[0]);
            f.render_widget(Manager::generate_machine_internals_block(), hlayout[1]);
            f.render_widget(Manager::generate_program_info_block(), vlayout[0]);
        }).unwrap();
    }

    fn generate_machine_display_block() -> Block<'static> {
        Block::default()
            .title("MACHINE DISPLAY")
            .borders(Borders::ALL)
    }

    fn generate_machine_internals_block() -> Block<'static> {
        Block::default()
            .title("MACHINE INTERNALS")
            .borders(Borders::ALL)
    }

    fn generate_program_info_block() -> Block<'static> {
        Block::default()
            .title("PROGRAM INFO")
            .borders(Borders::ALL)
    }
}
