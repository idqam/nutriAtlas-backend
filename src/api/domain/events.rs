use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    meal::{FoodLogEntry, LogEntryId},
    targets::DailyTargets,
    user::UserId,
};

pub const DOMAIN_EVENT_SCHEMA_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub event_id: Uuid,
    pub user_id: UserId,
    pub occurred_at: DateTime<Utc>,
    pub schema_version: u16,
}

impl EventEnvelope {
    pub fn new(user_id: UserId) -> Self {
        Self {
            event_id: Uuid::new_v4(),
            user_id,
            occurred_at: Utc::now(),
            schema_version: DOMAIN_EVENT_SCHEMA_VERSION,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoodLogged {
    pub envelope: EventEnvelope,
    pub entry: FoodLogEntry,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoodLogEdited {
    pub envelope: EventEnvelope,
    pub entry: FoodLogEntry,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct FoodLogDeleted {
    pub envelope: EventEnvelope,
    pub entry_id: LogEntryId,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct TargetsChanged {
    pub envelope: EventEnvelope,
    pub targets: DailyTargets,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event_type", content = "payload", rename_all = "snake_case")]
pub enum DomainEvent {
    FoodLogged(FoodLogged),
    FoodLogEdited(FoodLogEdited),
    FoodLogDeleted(FoodLogDeleted),
    TargetsChanged(TargetsChanged),
}
