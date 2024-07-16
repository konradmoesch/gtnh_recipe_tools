use std::fmt::{Debug, Display, Formatter};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct Item {
    #[serde(rename = "a")]
    pub amount: usize,
    #[serde(rename = "uN")]
    pub unlocalized_name: Option<String>,
    #[serde(rename = "lN")]
    pub localized_name: Option<String>
}

impl Item {
    pub fn new(unlocalized_name: &Option<String>, localized_name: &Option<String>) -> Self {
        Self {
            amount: 0,
            unlocalized_name: unlocalized_name.clone(),
            localized_name: localized_name.clone(),
        }
    }
    pub fn get_name(&self) -> String {
        let name = self.localized_name.clone().unwrap_or(self.unlocalized_name.clone().unwrap_or("ERROR_NAME_NOT_FOUND".to_string()));
        name
    }
}

impl Display for Item{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let name = self.get_name();
        write!(f, "{}x {}", self.amount, name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let only_unlocalized_name = Item {
            amount: 0,
            unlocalized_name: Some("testitem".to_string()),
            localized_name: None,
        };
        assert_eq!(only_unlocalized_name.get_name(), "testitem");
        let both_names = Item {
            amount: 0,
            unlocalized_name: Some("testitem".to_string()),
            localized_name: Some("Test Item".to_string()),
        };
        assert_eq!(both_names.get_name(), "Test Item");
    }
}
