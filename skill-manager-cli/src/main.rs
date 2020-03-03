use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use skill_manager::{
    employees::{
        in_memory::{EmployeeApi, EmployeeDb},
        EmployeeId, FirstName, LastName, ProjectContribution,
    },
    in_memory::in_memory_api_using,
    projects::{
        in_memory::{ProjectDb, ProjectsApi},
        ProjectDescription, ProjectId, ProjectLabel,
    },
    skills::{
        in_memory::{SkillDb, SkillsApi},
        usecase::{PageNumber, PageSize},
        SkillId, SkillLabel,
    },
};
use std::{cell::RefCell, env, fs, path::Path, process, rc::Rc};
use structopt::StructOpt;
use time::Date;

fn read_from<O: DeserializeOwned + Default>(path: impl AsRef<Path>) -> Result<O> {
    if let Ok(file_contents) = fs::read_to_string(path) {
        serde_json::from_str(&file_contents).map_err(Into::into)
    } else {
        Ok(Default::default())
    }
}

#[derive(StructOpt)]
enum Opt {
    Skill(SkillCommand),
    Project(ProjectCommand),
    Employee(EmployeeCommand),
}

#[derive(StructOpt)]
enum SkillCommand {
    Add {
        label: SkillLabel,
    },
    Find {
        #[structopt(short = "p", long = "page")]
        page: Option<PageNumber>,
        #[structopt(short = "s", long = "size")]
        page_size: Option<PageSize>,
    },
    Get {
        id: SkillId,
    },
    Delete {
        id: SkillId,
    },
}

#[derive(StructOpt)]
enum ProjectCommand {
    Add {
        #[structopt(short = "l", long = "label")]
        label: ProjectLabel,
        #[structopt(short = "d", long = "description")]
        description: ProjectDescription,
    },
    Delete {
        id: ProjectId,
    },
    Get {
        id: ProjectId,
    },
}

#[derive(StructOpt)]
enum EmployeeCommand {
    Add {
        #[structopt(short = "f", long = "first-name")]
        first_name: FirstName,
        #[structopt(short = "l", long = "last-name")]
        last_name: LastName,
    },
    Delete {
        id: EmployeeId,
    },
    Get {
        id: EmployeeId,
    },
    AssignProject {
        #[structopt(short = "p", long = "project-id")]
        project_id: ProjectId,
        #[structopt(short = "d", long = "start-date", parse(try_from_str = parse_date))]
        start_date: Date,
        #[structopt(long = "end-date", parse(try_from_str = parse_date))]
        end_date: Option<Date>,
        contribution: ProjectContribution,
    },
}

fn parse_date(s: &str) -> Result<Date> {
    time::parse(s, "%F").map_err(Into::into)
}

fn main() {
    let skill_db: SkillDb = Rc::new(RefCell::new(read_from("./skills.json").unwrap()));
    let project_db: ProjectDb = Rc::new(RefCell::new(read_from("./projects.json").unwrap()));
    let employee_db: EmployeeDb = Rc::new(RefCell::new(read_from("./employees.json").unwrap()));

    let api = in_memory_api_using(skill_db.clone(), project_db.clone(), employee_db.clone());

    let command = Opt::from_args();

    if let Err(e) = match command {
        Opt::Skill(skill_command) => skill_op(skill_command, api.skills),
        Opt::Project(project_command) => project_op(project_command, api.projects),
        Opt::Employee(employee_command) => employee_op(employee_command, api.employees),
    } {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn skill_op(skill_command: SkillCommand, api: SkillsApi) -> Result<()> {
    match skill_command {
        SkillCommand::Add { label } => {
            let added_skill = (api.add)(label).context("")?;
            println!("Added skill {:?}", added_skill);
            Ok(())
        }
        _ => Ok(()),
    }
}

fn project_op(project_command: ProjectCommand, api: ProjectsApi) -> Result<()> {
    Ok(())
}

fn employee_op(employee_command: EmployeeCommand, api: EmployeeApi) -> Result<()> {
    Ok(())
}
