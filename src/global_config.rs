use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

use crate::constants::CONFIG_DIR;

#[derive(Serialize, Deserialize, Debug)]
pub struct GlobalConfig {
  pub current_project: Option<String>,
}

impl GlobalConfig {
  pub fn new() -> GlobalConfig {
    GlobalConfig { current_project: None }
  }

  pub fn load() -> Result<GlobalConfig, Box<dyn std::error::Error>> {
    let file_path = Path::new(CONFIG_DIR);
    if !file_path.exists() { return Ok(GlobalConfig::new()); }
    let contents = fs::read_to_string(file_path)?;
    let config: GlobalConfig = serde_yaml::from_str(&contents)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(config)
  }

  pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
    let file_path = Path::new(CONFIG_DIR);
    let contents = serde_yaml::to_string(&self)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    fs::write(file_path, contents)?;
    Ok(())
  }

  pub fn set_current_project(&mut self, project_name: String) -> Result<(), Box<dyn std::error::Error>> {
    self.current_project = Some(project_name);
    self.save()?;
    Ok(())
  }
}