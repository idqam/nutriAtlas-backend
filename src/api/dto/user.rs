use chrono::NaiveDate;
use chrono::Utc;
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};

use crate::api::domain::user::{ActivityLevel, Goal, Plan, Profile, Role, Sex, User, UserId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnitPreference {
    Metric,
    Us,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserResponse {
    pub user_id: UserId,
    pub role: Role,
    pub plan: Plan,
    pub timezone: Tz,
    pub pref_units: UnitPreference,
    pub goals: Vec<Goal>,
    pub profile: Option<ProfileResponse>,
}

impl From<&User> for UserResponse {
    fn from(user: &User) -> Self {
        Self {
            user_id: user.user_id,
            role: user.role,
            plan: user.plan,
            timezone: user.timezone,
            pref_units: match user.pref_units.as_str() {
                "us" => UnitPreference::Us,
                _ => UnitPreference::Metric,
            },
            goals: user.goals.clone(),
            profile: user.profile.as_ref().map(ProfileResponse::from),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ProfileResponse {
    pub sex: Option<Sex>,
    pub birth_date: Option<NaiveDate>,
    pub height_cm: Option<f64>,
    pub current_weight_kg: Option<f64>,
    pub activity_level: Option<ActivityLevel>,
}

impl From<&Profile> for ProfileResponse {
    fn from(profile: &Profile) -> Self {
        Self {
            sex: profile.sex,
            birth_date: profile.birth_date,
            height_cm: profile.body_metrics.height_cm,
            current_weight_kg: profile.body_metrics.current_weight_kg,
            activity_level: profile.body_metrics.activity_level,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateProfileRequest {
    pub sex: Option<Sex>,
    pub birth_date: Option<NaiveDate>,
    pub height_cm: Option<f64>,
    pub activity_level: Option<ActivityLevel>,
}

impl UpdateProfileRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self
            .birth_date
            .is_some_and(|birth_date| birth_date > Utc::now().date_naive())
        {
            return Err("birth_date cannot be in the future".into());
        }
        if self
            .height_cm
            .is_some_and(|height| !height.is_finite() || height <= 0.0)
        {
            return Err("height_cm must be finite and positive".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateUserSettingsRequest {
    pub timezone: String,
    pub pref_units: UnitPreference,
}

impl UpdateUserSettingsRequest {
    pub fn parsed_timezone(&self) -> Result<Tz, String> {
        self.timezone
            .parse()
            .map_err(|_| format!("invalid timezone: {}", self.timezone))
    }
}
