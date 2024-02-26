use serde::{Deserialize, Serialize};
use std::{fs, io};
use std::path::Path;
use std::fmt;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
  pub name: String,
  pub path: Option<String>,
  pub editor: Option<String>,
  pub terminal: Option<String>,
}

impl ProjectConfig {
  pub fn new(name: &str) -> ProjectConfig {
    ProjectConfig {
      name: name.to_string(),
      path: None,
      editor: None,
      terminal: None,
    }
  }

  pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&contents)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    Ok(config)
  }

  pub fn save(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{}.yml", self.name);
    let file_path = path.join(&file_name);

    if !path.exists() {
      fs::create_dir_all(path)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    }

    if file_path.exists() {
      return Err(Box::new(io::Error::new(io::ErrorKind::AlreadyExists, format!("File '{}' already exists.", file_name))));
    }
    let contents = serde_yaml::to_string(&self)
      .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
    fs::write(file_path, contents)?;
    Ok(())
  }
}

impl fmt::Display for ProjectConfig {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "ProjectConfig {{ name: {}", self.name)?;
    if let Some(ref path) = self.path {
      write!(f, ", path: {}", path)?;
    }
    if let Some(ref editor) = self.editor {
      write!(f, ", editor: {}", editor)?;
    }
    if let Some(ref terminal) = self.terminal {
      write!(f, ", terminal: {}", terminal)?;
    }
    write!(f, " }}")
  }
}