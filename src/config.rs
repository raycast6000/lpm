use std::fs::write;
use serde::{Serialize, Deserialize};
use serde_json::to_string_pretty;

#[derive(Serialize, Deserialize)]
struct Dependency {
  name: String,
  version: String
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
  name: String,
  version: String,
  authors: Vec<String>,
  dependencies: Vec<Dependency>
}

impl ProjectConfig {
  fn new(name: &str) -> Self {
    ProjectConfig { name: name.to_string(), version: "0.1.0".to_string(), authors: vec![], dependencies: vec![] }
  }
}

pub fn create_default_config(config_path: &str, project_name: &str) {
  let conf: ProjectConfig = ProjectConfig::new(project_name);
  
  write(config_path, to_string_pretty(&conf).unwrap()).unwrap();
}