use std::fs::write;
use serde::{Deserialize, Serialize};
use serde_json::to_string;

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectConfig {
  name: String,
  authors: Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct LusrConfig {
  auth_token: String,
}

impl LusrConfig {
  fn new(auth_token: String) -> Self {
    LusrConfig {
      auth_token
    }
  }

  fn _set_auth_key(mut self, auth_token: String) {
    self.auth_token = auth_token;
  }
}

pub fn auth(path: &str, auth_token: String) {
  write(path, to_string(&LusrConfig::new(auth_token)).unwrap()).unwrap();

  println!("Authentication token successfully added.");
}