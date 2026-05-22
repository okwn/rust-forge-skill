use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl Item {
    pub fn new(name: String, description: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            description,
            created_at: chrono::Utc::now(),
        }
    }

    pub fn validate(&self) -> Result<(), crate::error::ValidationError> {
        if self.name.trim().is_empty() {
            return Err(crate::error::ValidationError::EmptyName);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_creation() {
        let item = Item::new("Test".into(), None);
        assert_eq!(item.name, "Test");
    }

    #[test]
    fn test_item_validate() {
        let item = Item::new("Test".into(), None);
        assert!(item.validate().is_ok());

        let empty = Item::new("".into(), None);
        assert!(empty.validate().is_err());
    }
}