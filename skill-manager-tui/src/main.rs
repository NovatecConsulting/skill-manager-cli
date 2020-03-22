use std::{io, iter};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Alignment, Constraint, Direction, Layout},
    style::Style,
    widgets::{Block, Borders, Paragraph, Text, Widget},
    Terminal,
};

fn main() {
    if let Err(e) = go() {
        eprintln!("{}", e);
    }
}

type Result<T> = std::result::Result<T, io::Error>;

#[derive(Default)]
struct State {
    input: String,
    values: Vec<String>,
}

fn go() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let stdin = io::stdin();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;
    terminal.hide_cursor()?;

    let mut state = State::default();
    draw(&mut terminal, &state)?;

    for k in stdin.keys() {
        match k? {
            Key::Esc => {
                return Ok(());
            }
            Key::Char('\n') => {
                state.values.push(state.input.clone());
                state.input.clear();
            }
            Key::Char(c) => {
                state.input.push(c);
            }
            Key::Backspace => {
                state.input.pop();
            }
            _ => {}
        }
        draw(&mut terminal, &state)?;
    }

    Ok(())
}

fn draw(terminal: &mut Terminal<impl Backend>, state: &State) -> Result<()> {
    terminal.draw(|mut f| {
        let size = f.size();

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(size);

        Block::default()
            .title("Header")
            .borders(Borders::ALL)
            .render(&mut f, chunks[0]);

        let list: Vec<_> = state
            .values
            .iter()
            .map(|s| format!("{}\n", s))
            .map(Text::raw)
            .collect();
        Paragraph::new(list.iter())
            .block(Block::default().title("List").borders(Borders::ALL))
            .alignment(Alignment::Left)
            .wrap(false)
            .render(&mut f, chunks[1]);

        Paragraph::new(iter::once(&Text::raw(&state.input)))
            .block(Block::default().title("Input").borders(Borders::ALL))
            .style(Style::default())
            .alignment(Alignment::Left)
            .wrap(true)
            .render(&mut f, chunks[2]);
    })?;

    Ok(())
}
