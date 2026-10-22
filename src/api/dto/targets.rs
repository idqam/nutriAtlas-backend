use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::api::domain::targets::DailyTargets;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SetDailyTargetsRequest {
    pub effective_from: NaiveDate,
    pub calories_kcal: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
}

impl SetDailyTargetsRequest {
    pub fn into_domain(self) -> Result<DailyTargets, String> {
        let targets = DailyTargets {
            effective_from: self.effective_from,
            calories_kcal: self.calories_kcal,
            protein_g: self.protein_g,
            carbs_g: self.carbs_g,
            fat_g: self.fat_g,
        };
        targets.validate().map_err(|error| error.to_string())?;
        Ok(targets)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DailyTargetsResponse {
    pub effective_from: NaiveDate,
    pub calories_kcal: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
}

impl From<DailyTargets> for DailyTargetsResponse {
    fn from(targets: DailyTargets) -> Self {
        Self {
            effective_from: targets.effective_from,
            calories_kcal: targets.calories_kcal,
            protein_g: targets.protein_g,
            carbs_g: targets.carbs_g,
            fat_g: targets.fat_g,
        }
    }
}
