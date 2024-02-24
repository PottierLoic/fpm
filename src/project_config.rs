use serde::{Deserialize, Serialize};
use std::{fs, io};
use std::path::Path;

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

  pub fn load_from(path: &Path) -> io::Result<Self> {
    let contents = fs::read_to_string(path)?;
    let config = serde_yaml::from_str(&contents)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    Ok(config)
  }

  pub fn save_to(&self, path: &Path) -> io::Result<()> {
    let file_name = format!("{}.yml", self.name);
    let file_path = path.join(&file_name);

    if file_path.exists() {
      return Err(io::Error::new(io::ErrorKind::AlreadyExists, format!("File '{}' already exists.", file_name)))
    }

    let contents = serde_yaml::to_string(&self)
      .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    fs::write(file_path, contents)?;
    Ok(())
  }
}