use std::io;
use termion::raw::{IntoRawMode, RawTerminal};
use crate::chip::VM;
use itertools::Itertools;
use tui::{
    Terminal,
    backend::{Backend, TermionBackend},
    layout::{Corner, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols,
    text::{Span, Spans},
    widgets::canvas::{Canvas, Context, Line, Map, MapResolution, Rectangle},
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
        let vlayout = Manager::generate_vertical_layout();
        let hlayout = Manager::generate_horizontal_layout();

        let machine_info_block    = Manager::generate_machine_internals_block(machine);
        let program_info_block    = Manager::generate_program_info_block(machine);
        let machine_display_block = Manager::generate_machine_display_block(machine);

        machine.display_man.terminal.draw(move |f| {
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

    fn generate_machine_display_block(machine: &VM) -> Canvas<'static, impl Fn(&mut Context<'_>)> {
        let gfx_memory_cpy = machine.gfx_memory.clone();
        let canvas = Canvas::default()
            .block(Block::default().title("CHIP8 DISPLAY").borders(Borders::ALL))
            .x_bounds([0.0, 64.0])
            .y_bounds([0.0, 32.0])
            .marker(symbols::Marker::Block)
            .background_color(Color::Red)
            .paint(move |ctx| {
                for (line_index, line) in gfx_memory_cpy.iter().enumerate() {
                    for pixel_index in 0..64 {
                        let pixel = (*line & (0x1 << pixel_index)) >> pixel_index;
                        ctx.draw(&Rectangle{
                            x: pixel_index as f64,
                            y: line_index as f64,
                            width: 1.0,
                            height: 1.0,
                            color: if pixel > 0 { Color::White } else { Color::Blue }
                        });
                    }
                }
            });

        return canvas;
    }

    fn generate_machine_internals_block(machine: &VM) -> List<'static> {
        let machine_pc = machine.pc.clone();
        let memory_window_range = machine_pc..[machine_pc, machine_pc - 10].iter().min().unwrap() + 10;
        let memory_window = &machine.memory[memory_window_range];

        let mut memory_window_list: Vec<ListItem> = Vec::with_capacity(10);
        for (upper, lower) in memory_window.iter().tuples() {
            memory_window_list.push(
                ListItem::new(format!("{:#08x}", (upper << 8) & lower))
                .style(Style::default().fg(Color::Yellow))
            );
        }

        return List::new(memory_window_list)
            .block(Block::default().borders(Borders::ALL).title("MACHINE INFO"))
            .start_corner(Corner::BottomLeft)
            .highlight_style(
                Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol(">> ");
    }

    fn generate_program_info_block(_machine: &VM) -> Block<'static> {
        Block::default()
            .title("PROGRAM INFO")
            .borders(Borders::ALL)
    }
}
