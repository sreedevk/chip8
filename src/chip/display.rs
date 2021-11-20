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

        /* Frame Loading + Rendering */
        let outline = Manager::generate_outline();
        let vlayout = Manager::generate_vertical_layout(); // .split(f.size());
        let hlayout = Manager::generate_horizontal_layout(); // .split(vlayout[1]);

        let machine_info_block = Manager::generate_machine_internals_block(machine);
        let machine_display_block = Manager::generate_machine_display_block(machine);
        let program_info_block = Manager::generate_program_info_block(machine);

        machine.display_man.terminal.draw(|f| {
            let vlayout_vec = vlayout.split(f.size());
            let hlayout_vec = hlayout.split(vlayout_vec[1]);

            f.render_widget(outline, f.size());
            f.render_widget(program_info_block, vlayout_vec[0]);
            f.render_widget(machine_display_block, hlayout_vec[0]);
            f.render_widget(machine_info_block, hlayout_vec[1]);
        }).unwrap();
    }

    fn generate_outline() -> Block<'static> {
        return Block::default()
            .title("Chip8")
            .borders(Borders::ALL);
    }

    fn generate_vertical_layout() -> Layout {
        return Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(10), Constraint::Percentage(90)].as_ref());
    }

    fn generate_horizontal_layout() -> Layout {
        return Layout::default()
            .direction(Direction::Horizontal)
            .vertical_margin(0)
            .horizontal_margin(1)
            .constraints([
                Constraint::Percentage(70),
                Constraint::Percentage(30)
            ].as_ref());
    }

    fn generate_machine_display_block(machine: &VM) -> Block<'static> {
        Block::default()
            .title("MACHINE DISPLAY")
            .borders(Borders::ALL)
    }

    fn generate_machine_internals_block(machine: &VM) -> Block<'static> {
        Block::default()
            .title("MACHINE INTERNALS")
            .borders(Borders::ALL)
    }

    fn generate_program_info_block(machine: &VM) -> Block<'static> {
        Block::default()
            .title("PROGRAM INFO")
            .borders(Borders::ALL)
    }
}
