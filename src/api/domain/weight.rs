use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::user::UserId;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct WeightEntry {
    pub user_id: UserId,
    pub kg: f64,
    pub measured_at: DateTime<Utc>,
}
