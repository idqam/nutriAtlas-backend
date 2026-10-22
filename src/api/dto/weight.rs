use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::api::domain::{
    user::UserId,
    weight::{WeightEntry, WeightEntryId},
};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateWeightEntryRequest {
    pub kg: f64,
    #[serde(default)]
    pub measured_at: Option<DateTime<Utc>>,
}

impl CreateWeightEntryRequest {
    pub fn into_domain(self, user_id: UserId) -> Result<WeightEntry, String> {
        WeightEntry::new(user_id, self.kg, self.measured_at.unwrap_or_else(Utc::now))
            .map_err(|error| error.to_string())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WeightEntryResponse {
    pub id: WeightEntryId,
    pub kg: f64,
    pub measured_at: DateTime<Utc>,
}

impl From<WeightEntry> for WeightEntryResponse {
    fn from(entry: WeightEntry) -> Self {
        Self {
            id: entry.id,
            kg: entry.kg,
            measured_at: entry.measured_at,
        }
    }
}
