use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
    #[serde(rename = "a")]
    pub amount: usize,
    #[serde(rename = "uN")]
    pub unlocalized_name: Option<String>,
    #[serde(rename = "lN")]
    pub localized_name: Option<String>
}

impl Display for Item{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = self.localized_name.clone().unwrap_or(self.unlocalized_name.clone().unwrap());
        write!(f, "{}x {}", self.amount, name)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Fluid {
    #[serde(rename = "a")]
    pub amount: usize,
    #[serde(rename = "uN")]
    pub unlocalized_name: Option<String>,
    #[serde(rename = "lN")]
    pub localized_name: Option<String>
}

impl Display for Fluid {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = self.localized_name.clone().unwrap_or(self.unlocalized_name.clone().unwrap());
        write!(f, "{}l {}", self.amount, name)
    }
}