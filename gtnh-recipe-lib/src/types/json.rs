use serde::{Deserialize, Serialize};
use crate::types::gregtech_machine::GregtechMachine;

#[derive(Serialize, Deserialize, Debug)]
pub struct Source {
    #[serde(rename = "type")]
    pub source_type: String,
    #[serde(default)]
    pub recipes: Vec<serde_json::Value>,
    #[serde(default)]
    pub machines: Vec<GregtechMachine>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonFormat{
    pub sources: Vec<Source>
}