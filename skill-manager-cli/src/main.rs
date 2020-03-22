use anyhow::Result;
use employees::EmployeeDb;
use serde::{de::DeserializeOwned, Serialize};
use skill_manager::{
    employees::{
        usecase::{
            AddEmployee, AssignProjectToEmployee, AssignSkillToEmployee, DeleteEmployeeById,
            GetEmployeeById, ProjectAssignmentRequest,
        },
        EmployeeId, FirstName, LastName, ProjectContribution, SkillLevel,
    },
    projects::{
        usecase::{AddProject, DeleteProject, GetProject},
        ProjectDescription, ProjectId, ProjectLabel,
    },
    skills::{
        usecase::{AddSkill, DeleteSkillById, FindSkills, GetSkillById, PageNumber, PageSize},
        SkillId, SkillLabel,
    },
};
use skill_manager_in_memory::{
    employees::{self},
    projects::ProjectDb,
    skills::SkillDb,
};
use std::{fs, fs::File, io, path::Path, process};
use structopt::StructOpt;
use time::Date;

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
        #[structopt(short = "e", long = "employee-id")]
        employee_id: EmployeeId,
        #[structopt(short = "p", long = "project-id")]
        project_id: ProjectId,
        #[structopt(short = "d", long = "start-date", parse(try_from_str = parse_date))]
        start_date: Date,
        #[structopt(long = "end-date", parse(try_from_str = parse_date))]
        end_date: Option<Date>,
        contribution: ProjectContribution,
    },
    AssignSkill {
        #[structopt(short = "e", long = "employee-id")]
        employee_id: EmployeeId,
        #[structopt(short = "s", long = "skill-id")]
        skill_id: SkillId,
        #[structopt(short = "l", long = "skill-level")]
        skill_level: SkillLevel,
    },
}

struct FileBackedDb<T> {
    db: T,
    file_path: Box<dyn AsRef<Path>>,
}

impl<T: Serialize + DeserializeOwned + Default> FileBackedDb<T> {
    fn from_path(file_path: Box<dyn AsRef<Path>>) -> Result<Self> {
        let val = match fs::read_to_string(&*file_path) {
            Ok(file_contents) => serde_json::from_str(&file_contents)?,
            _ => Default::default(),
        };
        Ok(FileBackedDb { db: val, file_path })
    }
    fn persist(self) -> Result<()> {
        let file = File::create(&*self.file_path)?;
        serde_json::to_writer_pretty(io::BufWriter::new(file), &self.db)?;
        Ok(())
    }
}

fn parse_date(s: &str) -> Result<Date> {
    time::parse(s, "%F").map_err(Into::into)
}

fn main() {
    let skill_db: FileBackedDb<SkillDb> =
        FileBackedDb::from_path(Box::new("./skills.json")).unwrap();
    let project_db: FileBackedDb<ProjectDb> =
        FileBackedDb::from_path(Box::new("./projects.json")).unwrap();
    let employee_db: FileBackedDb<EmployeeDb> =
        FileBackedDb::from_path(Box::new("./employees.json")).unwrap();

    let command = Opt::from_args();

    if let Err(e) = match command {
        Opt::Skill(skill_command) => skill_op(skill_command, skill_db),
        Opt::Project(project_command) => project_op(project_command, project_db),
        Opt::Employee(employee_command) => {
            employee_op(employee_command, employee_db, project_db, skill_db)
        }
    } {
        eprintln!("Error: {}", e);
        process::exit(1);
    }
}

fn print_json(val: &impl Serialize) {
    println!("{}", serde_json::to_string_pretty(val).unwrap());
}

fn skill_op(skill_command: SkillCommand, mut skill_db: FileBackedDb<SkillDb>) -> Result<()> {
    match skill_command {
        SkillCommand::Add { label } => {
            let added_skill = skill_db.db.add(label)?;
            print_json(&added_skill)
        }
        SkillCommand::Get { id } => {
            let skill = skill_db.db.get(id)?;
            print_json(&skill)
        }
        SkillCommand::Find { page, page_size } => {
            let found = skill_db.db.find(page, page_size)?;
            print_json(&found)
        }
        SkillCommand::Delete { id } => {
            skill_db.db.delete(id.clone())?;
            print_json(&format!("Deleted skill {}", id))
        }
    }
    skill_db.persist()
}

fn project_op(
    project_command: ProjectCommand,
    mut project_db: FileBackedDb<ProjectDb>,
) -> Result<()> {
    match project_command {
        ProjectCommand::Add { label, description } => {
            let added_project = project_db.db.add(label, description)?;
            print_json(&added_project)
        }
        ProjectCommand::Delete { id } => {
            project_db.db.delete(id.clone())?;
            print_json(&format!("Deleted project {}", id))
        }
        ProjectCommand::Get { id } => {
            let project = project_db.db.get(id)?;
            print_json(&project)
        }
    }
    project_db.persist()
}

fn employee_op(
    employee_command: EmployeeCommand,
    mut employee_db: FileBackedDb<EmployeeDb>,
    project_db: FileBackedDb<ProjectDb>,
    skill_db: FileBackedDb<SkillDb>,
) -> Result<()> {
    match employee_command {
        EmployeeCommand::Add {
            first_name,
            last_name,
        } => {
            let added = employee_db.db.add(first_name, last_name)?;
            print_json(&added)
        }
        EmployeeCommand::Delete { id } => {
            employee_db.db.delete(id.clone())?;
            print_json(&format!("Deleted employee {}", id))
        }
        EmployeeCommand::Get { id } => {
            let employee = employee_db.db.get(id)?;
            print_json(&employee)
        }
        EmployeeCommand::AssignProject {
            employee_id,
            project_id,
            start_date,
            end_date,
            contribution,
        } => {
            let assigned = employee_db.db.with(&project_db.db).assign_project(
                employee_id,
                ProjectAssignmentRequest {
                    project_id,
                    contribution,
                    start_date,
                    end_date,
                },
            )?;
            print_json(&assigned)
        }
        EmployeeCommand::AssignSkill {
            employee_id,
            skill_id,
            skill_level,
        } => {
            let assigned = employee_db.db.with(&skill_db.db).assign_skill(
                employee_id,
                skill_id,
                skill_level,
            )?;
            print_json(&assigned)
        }
    }
    employee_db.persist()
}
