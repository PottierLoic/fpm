use std::path::Path;
use std::process::Command;
use crate::global_config::GlobalConfig;
use crate::project_config::ProjectConfig;
use crate::constants::PROJECTS_DIR;

pub struct Fpm {
  pub global_config: GlobalConfig,
}

impl Fpm {
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    let global_config = GlobalConfig::load()?;
    Ok(Fpm { global_config })
  }

  pub fn create_new_project(&mut self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
      return Err("Usage: fpm new <project-name>".into());
    }

    let new_project = ProjectConfig::new(&args[0]);
    new_project.save(Path::new(PROJECTS_DIR))?;
    println!("Project '{}' created successfully.", new_project.name);
    self.global_config.set_current_project(new_project.name)?;
    Ok(())
  }

  pub fn configure_project(&self, args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    if args.is_empty() {
      return Err("Usage: fpm config --param1=value --param2=value ...".into());
    }

    if args.len() % 2 != 0 {
      return Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::InvalidInput,
        "Invalid arguments formatting, use “—parameter=value”.",
      )));
    }

    let project_name = match self.global_config.current_project.as_ref() {
      Some(name) => name,
      None => return Err(Box::new(std::io::Error::new(std::io::ErrorKind::NotFound, "No project selected"))),
    };

    let mut config = ProjectConfig::load(project_name)?;

    for idx in (0..args.len()).step_by(2) {
      let value = Some(args[idx + 1].clone());
      match args[idx].as_str() {
        "--editor" => { config.editor = value; },
        "--path" => { config.path = value; },
        "--terminal" => { config.terminal = value; },
        _ => {
          return Err("Invalid argument. Please check the help documentation or man for a list of available arguments.".into());
        },
      }
    }

    config.save(Path::new(PROJECTS_DIR))?;
    Ok(())
  }

  pub fn open_project(&self) -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(target_os = "windows")]
    let shell_command = "cmd";
    #[cfg(target_os = "windows")]
    let shell_flag = "/C";

    #[cfg(not(target_os = "windows"))]
    let shell_command = "sh";
    #[cfg(not(target_os = "windows"))]
    let shell_flag = "-c";

    match self.global_config.current_project.as_deref() {
      Some(project_name) => {
        let project_config = ProjectConfig::load(project_name)?;
        let project_path = project_config.path.unwrap_or_else(|| ".".to_string());

        // IDE part.
        if let Some(editor_command) = project_config.editor {
          Command::new(shell_command)
            .arg(shell_flag)
            .arg(format!("{} {}", editor_command, project_path))
            .spawn()?;
        } else {
          return Err("Editor not configured for this project".into())
        }

        // Terminal part.
        if let Some(terminal_command) = project_config.terminal {
          Command::new(shell_command)
            .arg(shell_flag)
            .arg(format!("cd {} && {}", project_path, terminal_command))
            .spawn()?;
        }
        Ok(())

      },
      None => return Err("No project selected".into()),
    }
  }
}
