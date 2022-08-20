use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use serde::Serialize;
use serde::Deserialize;

#[derive(Serialize, Deserialize)]
pub struct Translation {
   translate_result: Vec<Vec<HashMap<String, String>>>
}

impl Display for Translation {
   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.translate_result[0][0][&"tgt".to_string()])
   }
}