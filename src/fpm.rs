use std::path::Path;
use crate::global_config::GlobalConfig;
use crate::project_config::ProjectConfig;
use crate::constants::PROJECTS_DIR;

pub struct Fpm {
  pub global_config: GlobalConfig,
}

impl Fpm {
  pub fn new() -> Self {
    let global_config = match GlobalConfig::load() {
      Ok(config) => config,
      Err(e) => {
        println!("Error loading global config: {}", e);
        std::process::exit(1);
      }
    };
    Fpm { global_config }
  }

  pub fn create_new_project(&mut self, args: &[String]) {
    if args.is_empty() {
      println!("Usage: fpm new <project-name>");
      return;
    }

    let new_project = ProjectConfig::new(&args[0]);
    match new_project.save_to(Path::new(PROJECTS_DIR)) {
      Ok(_) => println!("Project '{}' created successfully.", new_project.name),
      Err(e) => {
        println!("Error creating project: {}", e);
        return;
      }
    }
    self.global_config.set_current_project(new_project.name);
    self.global_config.save().expect("Failed to save global configuration.");
  }

  pub fn configure_project(&self, args: &[String]) {
    if args.is_empty() {
      println!("Usage: fpm config --param1=value --param2=value ...");
      return;
    }

    println!("Configuring project with args: {:?}", args);
  }

  pub fn open_project(&self) {
    println!("Opening project: {}", self.global_config.current_project.as_deref().unwrap_or("No project selected"));
  }
}

