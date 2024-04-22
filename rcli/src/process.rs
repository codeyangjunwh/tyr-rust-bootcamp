use serde::{Deserialize, Serialize};
use std::fs;
use csv::Reader;

#[derive(Debug, Serialize, Deserialize)]
struct Player {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8
}

pub fn process_csv(input: &str, output: &str) -> Result<()>{
  let mut reader = Reader::from_path(input)?;
  let mut ret: Vec<Player> = Vec::with_capacity(128);

  for result in reader.deserialize() {
      let record: Player = result?;
      ret.push(record);
  }

  let json = serde_json::to_string_pretty(&ret)?;
  fs::write(output, json)?;
  Ok(());
}