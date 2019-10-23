extern crate amelioration;

// use ameloration::{};

use std::io;

use amelioration::util::event::{Event, Events};
use chrono::{TimeZone, Utc};
use termion::event::Key;
use termion::input::MouseTerminal;
use termion::raw::IntoRawMode;
use termion::screen::AlternateScreen;
use tui::backend::TermionBackend;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Axis, Block, Borders, Chart, Dataset, Marker, Widget};
use tui::Terminal;

struct App {
    weight_at_time: Vec<(f64, f64)>,
    window: [f64; 2],
}

impl App {
    fn new(data: Vec<(f64, f64)>) -> App {
        App {
            weight_at_time: data,
            window: [1570967475.0, 1571745075.0],
        }
    }
}

// one day => 86400.000

fn main() -> Result<(), failure::Error> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.hide_cursor()?;

    let events = Events::new();

    let data: Vec<(f64, f64)> = vec![
        (1570967475.0, 176.2),
        (1571053875.0, 177.2),
        (1571140275.0, 174.6),
        (1571226675.0, 174.8),
        (1571313075.0, 176.4),
        (1571399475.0, 175.6),
        (1571658675.0, 176.6),
        (1571745075.0, 174.8),
    ];

    let dates: Vec<String> = data
        .iter()
        .map(|(d, _)| {
            Utc.timestamp((d).round() as i64, 0)
                .format("%a %b %e")
                .to_string()
        })
        .collect();

    let app = App::new(data);

    loop {
        terminal.draw(|mut f| {
            let size = f.size();
            Chart::default()
                .block(
                    Block::default()
                        .title("Chart")
                        .title_style(Style::default().fg(Color::Black).modifier(Modifier::BOLD))
                        .borders(Borders::ALL),
                )
                .x_axis(
                    Axis::default()
                        .title("Date")
                        .style(Style::default().fg(Color::Gray))
                        .labels_style(Style::default().modifier(Modifier::ITALIC))
                        .bounds(app.window)
                        .labels(&dates),
                )
                .y_axis(
                    Axis::default()
                        .title("Weight")
                        .style(Style::default().fg(Color::Gray))
                        .labels_style(Style::default().modifier(Modifier::ITALIC))
                        .bounds([170.0, 180.0])
                        .labels(&["170", "175", "180"]),
                )
                .datasets(&[Dataset::default()
                    .name("Weight over time")
                    .marker(Marker::Dot)
                    .style(Style::default().fg(Color::Red))
                    .data(app.weight_at_time.as_slice())])
                .render(&mut f, size);
        })?;

        match events.next()? {
            Event::Input(input) => {
                if input == Key::Char('q') {
                    break;
                }
            }
            Event::Tick => (),
        }
    }

    Ok(())
}
