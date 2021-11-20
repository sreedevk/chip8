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
        machine.display_man.terminal.draw(|f| { Manager::draw_layout(f) }).unwrap();
        // self.terminal.draw(|f| { Manager::draw_machine_display(f) }).unwrap();
    }

    pub fn draw_machine_display<B>(f: &mut Frame<B>)
    where 
        B: Backend,
    {}

    pub fn draw_layout<B>(f: &mut Frame<B>)
    where 
        B: Backend,
    {
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
            .margin(1)
            .constraints([
                Constraint::Percentage(60),
                Constraint::Percentage(40)
            ].as_ref())
            .split(vlayout[1]);

        let machine_display = Block::default()
            .title("MACHINE DISPLAY")
            .borders(Borders::ALL);

        let machine_stats = Block::default()
            .title("MACHINE INTERNALS")
            .borders(Borders::ALL);

        let machine_title = Block::default()
            .title("PROGRAM INFO")
            .borders(Borders::ALL);

        f.render_widget(main, f.size());
        f.render_widget(machine_display, hlayout[0]);
        f.render_widget(machine_stats, hlayout[1]);
        f.render_widget(machine_title, vlayout[0]);
    }
}
