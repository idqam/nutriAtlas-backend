use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WeightEntryId(pub Uuid);

impl WeightEntryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for WeightEntryId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WeightEntry {
    pub id: WeightEntryId,
    pub user_id: UserId,
    pub kg: f64,
    pub measured_at: DateTime<Utc>,
}

impl WeightEntry {
    pub fn new(
        user_id: UserId,
        kg: f64,
        measured_at: DateTime<Utc>,
    ) -> Result<Self, WeightEntryError> {
        let entry = Self {
            id: WeightEntryId::new(),
            user_id,
            kg,
            measured_at,
        };
        entry.validate()?;
        Ok(entry)
    }

    pub fn validate(&self) -> Result<(), WeightEntryError> {
        if !self.kg.is_finite() || self.kg <= 0.0 {
            return Err(WeightEntryError { kg: self.kg });
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WeightEntryError {
    pub kg: f64,
}

impl std::fmt::Display for WeightEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "weight must be finite and positive, got {} kg", self.kg)
    }
}

impl std::error::Error for WeightEntryError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creates_identified_positive_weight_entries() {
        let entry = WeightEntry::new(UserId::new(), 72.5, Utc::now()).unwrap();
        assert!(entry.kg.is_finite());
        assert_ne!(entry.id, WeightEntryId(Uuid::nil()));
    }

    #[test]
    fn rejects_non_positive_and_non_finite_weights() {
        for kg in [0.0, -1.0, f64::NAN, f64::INFINITY] {
            assert!(WeightEntry::new(UserId::new(), kg, Utc::now()).is_err());
        }
    }
}
