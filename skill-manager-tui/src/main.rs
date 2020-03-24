use anyhow::anyhow;
use skill_manager::{
    employees::{usecase::AddEmployee, FirstName, LastName},
    projects::{usecase::AddProject, ProjectDescription, ProjectLabel},
    skills::{usecase::AddSkill, SkillLabel},
};
use skill_manager_in_memory::{employees::EmployeeDb, projects::ProjectDb, skills::SkillDb};
use std::{
    fmt::{self, Display},
    io, iter,
};
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
    symbols::DOT,
    widgets::{Block, Borders, Paragraph, Tabs, Text, Widget},
    Terminal,
};

fn main() {
    if let Err(e) = go() {
        eprintln!("{}", e);
    }
}

type Result<T> = anyhow::Result<T>;

enum DataTab {
    Skills,
    Projects,
    Employees,
}

impl Display for DataTab {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                DataTab::Employees => "Employees",
                DataTab::Projects => "Projects",
                DataTab::Skills => "Skills",
            }
        )
    }
}

enum InputMode {
    List,
    Input(String),
}

struct State {
    open_tab: DataTab,
    mode: InputMode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open_tab: DataTab::Employees,
            mode: InputMode::List,
        }
    }
}

impl State {
    fn handle_input(&mut self, key: Key) -> Result<Vec<Effect>> {
        let mut effects = vec![];
        match &mut self.mode {
            InputMode::Input(input) => match key {
                Key::Esc => self.mode = InputMode::List,
                Key::Char('\n') => {
                    effects.push(Effect::SendInput(input.clone()));
                    self.mode = InputMode::List;
                }
                Key::Char(c) => {
                    input.push(c);
                }
                _ => {}
            },
            InputMode::List => match key {
                Key::Char('p') => {
                    self.open_tab = DataTab::Projects;
                }
                Key::Char('s') => {
                    self.open_tab = DataTab::Skills;
                }
                Key::Char('e') => {
                    self.open_tab = DataTab::Employees;
                }
                Key::Char('+') => self.mode = InputMode::Input(String::new()),
                Key::Esc => effects.push(Effect::Quit),
                _ => {}
            },
        }
        Ok(effects)
    }
}

enum Effect {
    SendInput(String),
    Quit,
}

#[derive(Default)]
struct Db {
    skills: SkillDb,
    projects: ProjectDb,
    employees: EmployeeDb,
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
    let mut db = Db::default();
    draw(&mut terminal, &state, &db)?;

    for k in stdin.keys() {
        let effects = state.handle_input(k?)?;
        for effect in effects {
            match effect {
                Effect::Quit => return Ok(()),
                Effect::SendInput(input) => match state.open_tab {
                    DataTab::Skills => {
                        db.skills.add(SkillLabel(input))?;
                    }
                    DataTab::Employees => {
                        create_employee(&mut db, input)?;
                    }
                    DataTab::Projects => {
                        db.projects
                            .add(ProjectLabel(input), ProjectDescription("".into()))?;
                    }
                },
            }
        }
        draw(&mut terminal, &state, &db)?;
    }

    Ok(())
}

fn create_employee(db: &mut Db, input: String) -> Result<()> {
    let mut words = input.split_whitespace();
    let first_name = words.next().ok_or(anyhow!("Empty first name"))?;
    let last_name = LastName(words.collect());
    db.employees
        .add(FirstName(first_name.to_string()), last_name)?;
    Ok(())
}

fn draw(terminal: &mut Terminal<impl Backend>, state: &State, db: &Db) -> Result<()> {
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

        Tabs::default()
            .block(Block::default().title("Skill Manager"))
            .titles(&["[E]mployees", "[P]rojects", "[S]kills"])
            .divider(DOT)
            .render(&mut f, chunks[0]);

        let list: Vec<_> = match state.open_tab {
            DataTab::Skills => db
                .skills
                .0
                .values()
                .map(|s| format!("{}\n", s.label))
                .collect(),
            DataTab::Projects => db
                .projects
                .0
                .values()
                .map(|p| format!("{}\n", p.label))
                .collect(),
            DataTab::Employees => db
                .employees
                .0
                .values()
                .map(|e| format!("{} {}\n", e.first_name, e.last_name))
                .collect(),
        };
        let list: Vec<_> = list.iter().map(Text::raw).collect();
        Paragraph::new(list.iter())
            .block(
                Block::default()
                    .title(&state.open_tab.to_string())
                    .borders(Borders::ALL),
            )
            .alignment(Alignment::Left)
            .wrap(false)
            .render(&mut f, chunks[1]);

        let (input, input_block) = match &state.mode {
            InputMode::Input(i) => (
                &i[..],
                Block::default().title("Input").borders(Borders::ALL),
            ),
            _ => ("", Block::default()),
        };
        Paragraph::new(iter::once(&Text::raw(input)))
            .block(input_block)
            .style(Style::default())
            .alignment(Alignment::Left)
            .wrap(true)
            .render(&mut f, chunks[2]);
    })?;

    Ok(())
}
