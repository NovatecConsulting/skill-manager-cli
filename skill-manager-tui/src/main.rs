use anyhow::anyhow;
use skill_manager::{
    employees::{
        usecase::{AddEmployee, AddEmployeeRequest, FindEmployees},
        EmailAddress, FirstName, LastName, TelephoneNumber, Title,
    },
    projects::{
        usecase::{AddProject, FindProjects},
        ProjectDescription, ProjectLabel,
    },
    skills::{
        usecase::{AddSkill, FindSkills},
        SkillLabel,
    },
};
use skill_manager_in_memory::{employees::EmployeeDb, projects::ProjectDb, skills::SkillDb};
use std::{
    fmt::{self, Display},
    io,
};
use termion::{
    event::Key,
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
};
use tui::{
    backend::{Backend, TermionBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, Paragraph, Tabs, Text, Widget},
    Frame, Terminal,
};

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

type Result<T> = anyhow::Result<T>;

#[derive(Copy, Clone)]
enum Header {
    Skills,
    Projects,
    Employees,
}

impl Header {
    const SIZE: u16 = 3;

    fn new(selected_tab: &Header) -> Tabs<&'static str> {
        Tabs::default()
            .block(Block::default().title("Skill Manager"))
            .titles(&["[E]mployees", "[P]rojects", "[S]kills"])
            .select(selected_tab.ix())
            .highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
    }
    fn ix(&self) -> usize {
        match self {
            Header::Skills => 2,
            Header::Projects => 1,
            Header::Employees => 0,
        }
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Header::Employees => "Employees",
                Header::Projects => "Projects",
                Header::Skills => "Skills",
            }
        )
    }
}

struct List {
    title: String,
    data: Vec<String>,
}

impl List {
    fn new(data: Vec<String>, state: &State) -> Self {
        Self {
            data,
            title: state.open_tab.to_string(),
        }
    }
    fn render<'a>(&self, mut f: &mut Frame<impl Backend>, chunk: Rect) {
        let list: Vec<_> = self.data.iter().map(Text::raw).collect();
        Paragraph::new(list.iter())
            .block(Block::default().title(&self.title))
            .alignment(Alignment::Left)
            .wrap(false)
            .render(&mut f, chunk);
    }
}

struct InputField<'a> {
    input: &'a str,
    open_tab: Header,
}

impl<'a> InputField<'a> {
    const SIZE: u16 = 1;
    fn new(input: &'a str, open_tab: Header) -> Self {
        Self { input, open_tab }
    }

    fn render(&mut self, mut f: &mut Frame<impl Backend>, chunk: Rect) -> (u16, u16) {
        let paragraph_chunk = chunk;
        let prefix = match &self.open_tab {
            Header::Skills => "New Skill: ",
            Header::Projects => "New Project: ",
            Header::Employees => "New Employee: ",
        };
        Paragraph::new([Text::raw(prefix), Text::raw(self.input)].iter())
            .style(Style::default())
            .alignment(Alignment::Left)
            .render(&mut f, chunk);

        (
            //TODO overflow alert!
            paragraph_chunk.left()
                + prefix.chars().count() as u16
                + self.input.chars().count() as u16,
            paragraph_chunk.top(),
        )
    }
}

struct HotkeyHelp;

impl HotkeyHelp {
    const SIZE: u16 = 1;
}

enum InputMode {
    List,
    Input(String),
}

struct State {
    open_tab: Header,
    mode: InputMode,
}

impl Default for State {
    fn default() -> Self {
        Self {
            open_tab: Header::Employees,
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
                Key::Backspace => {
                    input.pop();
                }
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
                    self.open_tab = Header::Projects;
                }
                Key::Char('s') => {
                    self.open_tab = Header::Skills;
                }
                Key::Char('e') => {
                    self.open_tab = Header::Employees;
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

fn run() -> Result<()> {
    let stdout = io::stdout().into_raw_mode()?;
    let stdout = MouseTerminal::from(stdout);
    let stdout = AlternateScreen::from(stdout);
    let stdin = io::stdin();
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // terminal.clear()?;
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
                    Header::Skills => {
                        db.skills.add(SkillLabel(input))?;
                    }
                    Header::Employees => {
                        create_employee(&mut db, input)?;
                    }
                    Header::Projects => {
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
    db.employees.add(AddEmployeeRequest {
        first_name: FirstName(first_name.into()),
        last_name,
        title: Title(String::new()),
        email: EmailAddress(String::new()),
        telephone: TelephoneNumber(String::new()),
    })?;
    Ok(())
}

fn draw(terminal: &mut Terminal<impl Backend>, state: &State, db: &Db) -> Result<()> {
    let mut set_cursor = None;
    terminal.draw(|mut f| {
        let size = f.size();

        let layout = match &state.mode {
            InputMode::List => vec![
                Constraint::Length(Header::SIZE),
                Constraint::Min(20),
                Constraint::Length(HotkeyHelp::SIZE),
            ],
            InputMode::Input(_) => vec![
                Constraint::Length(Header::SIZE),
                Constraint::Min(20),
                Constraint::Length(InputField::SIZE),
                Constraint::Length(HotkeyHelp::SIZE),
            ],
        };
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(layout)
            .split(size);

        let mut tabs = Header::new(&state.open_tab);
        tabs.render(&mut f, chunks[0]);

        let list = List::new(retrieve_data(&state, &db), &state);
        list.render(&mut f, chunks[1]);

        if let InputMode::Input(i) = &state.mode {
            let mut input_field = InputField::new(i, state.open_tab);
            let cursor_pos = input_field.render(&mut f, chunks[2]);
            set_cursor = Some(cursor_pos);

            //let paragraph_chunk = chunks[2];
            //Paragraph::new(iter::once(&Text::raw(i)))
            //    .block(Block::default().title("Input"))
            //    .style(Style::default())
            //    .alignment(Alignment::Left)
            //    .wrap(true)
            //    .render(&mut f, chunks[2]);

            //set_cursor = Some((
            //    //TODO overflow alert!
            //    paragraph_chunk.left() + i.len() as u16 + 1,
            //    paragraph_chunk.top() + 1,
            //));
        };

        let hotkey_hints = [Text::raw("[+]New entry")];
        let mut hotkey_helper = Paragraph::new(hotkey_hints.iter());
        let hotkey_chunk_ix = match &state.mode {
            InputMode::List => 2,
            InputMode::Input(_) => 3,
        };
        hotkey_helper.render(&mut f, chunks[hotkey_chunk_ix]);
    })?;

    if let Some((x, y)) = set_cursor {
        terminal.show_cursor()?;
        terminal.set_cursor(x, y)?;
    } else {
        terminal.hide_cursor()?;
    }

    Ok(())
}

fn retrieve_data(state: &State, db: &Db) -> Vec<String> {
    match state.open_tab {
        Header::Skills => db
            .skills
            .find_skills()
            .unwrap()
            .into_iter()
            .map(|s| format!("{}\n", s.label))
            .collect(),
        Header::Projects => db
            .projects
            .find_projects()
            .unwrap()
            .into_iter()
            .map(|p| format!("{}\n", p.label))
            .collect(),
        Header::Employees => db
            .employees
            .find_employees()
            .unwrap()
            .into_iter()
            .map(|e| format!("{} {}\n", e.first_name, e.last_name))
            .collect(),
    }
}
