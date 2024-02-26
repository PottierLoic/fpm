use std::path::Path;
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

    println!("Configuring project with args: {:?}", args);
    Ok(())
  }

  pub fn open_project(&self) -> Result<(), Box<dyn std::error::Error>> {
    match self.global_config.current_project.as_deref() {
      Some(project_name) => {
        println!("Opening project: {}", project_name);
        // TODO: Open the sortcuts here.
        Ok(())
      },
      None => Err("No project selected".into()),
    }
  }
}
