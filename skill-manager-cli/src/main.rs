use anyhow::Result;
use serde::{de::DeserializeOwned, Serialize};
use skill_manager::{
    employees::{
        usecase::{
            AddEmployee, AssignProjectToEmployee, AssignSkillToEmployee, DeleteEmployeeById,
            GetEmployeeById, ProjectAssignmentRequest,
        },
        EmployeeId, FirstName, LastName, ProjectContribution, SkillLevel,
    },
    projects::{ProjectDescription, ProjectId, ProjectLabel},
    skills::{
        usecase::{PageNumber, PageSize},
        SkillId, SkillLabel,
    },
};
use skill_manager_in_memory::{
    employees::{EmployeeApi, EmployeeStore},
    in_memory_api_using,
    projects::{ProjectStore, ProjectsApi},
    skills::{SkillStore, SkillsApi},
};
use std::{cell::RefCell, fs, fs::File, io, path::Path, process, rc::Rc};
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

struct FileBackedDb<T: Serialize> {
    db: Rc<RefCell<T>>,
    file_path: Box<dyn AsRef<Path>>,
}

impl<T: Serialize + DeserializeOwned + Default> FileBackedDb<T> {
    fn from_path(file_path: Box<dyn AsRef<Path>>) -> Result<Self> {
        let val = match fs::read_to_string(&*file_path) {
            Ok(file_contents) => serde_json::from_str(&file_contents)?,
            _ => Default::default(),
        };
        Ok(FileBackedDb {
            db: Rc::new(RefCell::new(val)),
            file_path,
        })
    }
}

impl<T: Serialize> Drop for FileBackedDb<T> {
    fn drop(&mut self) {
        if let Ok(file) = File::create(&*self.file_path) {
            serde_json::to_writer_pretty(io::BufWriter::new(file), &*self.db.borrow()).unwrap();
        }
    }
}

fn parse_date(s: &str) -> Result<Date> {
    time::parse(s, "%F").map_err(Into::into)
}

fn main() {
    let skill_db: FileBackedDb<SkillStore> =
        FileBackedDb::from_path(Box::new("./skills.json")).unwrap();
    let project_db: FileBackedDb<ProjectStore> =
        FileBackedDb::from_path(Box::new("./projects.json")).unwrap();
    let employee_db: FileBackedDb<EmployeeStore> =
        FileBackedDb::from_path(Box::new("./employees.json")).unwrap();

    let api = in_memory_api_using(
        skill_db.db.clone(),
        project_db.db.clone(),
        employee_db.db.clone(),
    );

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

fn print_json(val: &impl Serialize) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(val)?);
    Ok(())
}

fn skill_op(skill_command: SkillCommand, api: SkillsApi) -> Result<()> {
    match skill_command {
        SkillCommand::Add { label } => {
            let added_skill = (api.add)(label)?;
            print_json(&added_skill)
        }
        SkillCommand::Get { id } => {
            let skill = (api.get)(id)?;
            print_json(&skill)
        }
        SkillCommand::Find { page, page_size } => {
            let found = (api.find)(page, page_size)?;
            print_json(&found)
        }
        SkillCommand::Delete { id } => {
            (api.delete)(id.clone())?;
            print_json(&format!("Deleted skill {}", id))
        }
    }
}

fn project_op(project_command: ProjectCommand, api: ProjectsApi) -> Result<()> {
    match project_command {
        ProjectCommand::Add { label, description } => {
            let added_project = (api.add)(label, description)?;
            print_json(&added_project)
        }
        ProjectCommand::Delete { id } => {
            (api.delete)(id.clone())?;
            print_json(&format!("Deleted project {}", id))
        }
        ProjectCommand::Get { id } => {
            let project = (api.get)(id)?;
            print_json(&project)
        }
    }
}

fn employee_op(employee_command: EmployeeCommand, api: EmployeeApi) -> Result<()> {
    match employee_command {
        EmployeeCommand::Add {
            first_name,
            last_name,
        } => {
            let added = api.add(first_name, last_name)?;
            print_json(&added)
        }
        EmployeeCommand::Delete { id } => {
            api.delete(id.clone())?;
            print_json(&format!("Deleted employee {}", id))
        }
        EmployeeCommand::Get { id } => {
            let employee = api.get(id)?;
            print_json(&employee)
        }
        EmployeeCommand::AssignProject {
            employee_id,
            project_id,
            start_date,
            end_date,
            contribution,
        } => {
            let assigned = api.assign_project(
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
            let assigned = api.assign_skill(employee_id, skill_id, skill_level)?;
            print_json(&assigned)
        }
    }
}
