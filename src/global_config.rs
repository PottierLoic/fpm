use serde::{Deserialize, Serialize};
use std::{fs, io};
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

  pub fn load() -> io::Result<GlobalConfig> {
    let file_path = Path::new(CONFIG_DIR);
    if !file_path.exists() {
      return Ok(GlobalConfig::new());
    }
    let contents = fs::read_to_string(file_path)?;
    let config: GlobalConfig = serde_yaml::from_str(&contents)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(config)
  }

  pub fn save(&self) -> io::Result<()> {
    let file_path = Path::new(CONFIG_DIR);
    let contents = serde_yaml::to_string(&self)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    fs::write(file_path, contents)?;
    Ok(())
  }

  pub fn set_current_project(&mut self, project_name: String) {
    self.current_project = Some(project_name);
  }
}