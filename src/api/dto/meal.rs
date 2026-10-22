use crate::api::domain::{
    food::{FoodSource, NutrientProfile, ServingUnit},
    meal::{LogEntryId, MealType},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use super::common::FoodReference;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateFoodLogEntryRequest {
    pub food: FoodReference,
    pub unit: ServingUnit,
    pub quantity: f64,
    pub meal: MealType,
    #[serde(default)]
    pub consumed_at: Option<DateTime<Utc>>,
}

impl CreateFoodLogEntryRequest {
    pub fn validate(&self) -> Result<(), String> {
        if !self.quantity.is_finite() || self.quantity <= 0.0 {
            return Err("quantity must be finite and positive".into());
        }
        self.food.validate()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateFoodLogEntryRequest {
    pub expected_revision: u64,
    pub unit: ServingUnit,
    pub quantity: f64,
    pub meal: MealType,
    pub consumed_at: DateTime<Utc>,
}

impl UpdateFoodLogEntryRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.expected_revision == 0 {
            return Err("expected_revision must be greater than zero".into());
        }
        if !self.quantity.is_finite() || self.quantity <= 0.0 {
            return Err("quantity must be finite and positive".into());
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoodLogEntryResponse {
    pub id: LogEntryId,
    pub food: FoodReference,
    pub food_name: String,
    pub source: FoodSource,
    pub unit: ServingUnit,
    pub quantity: f64,
    pub grams: f64,
    pub nutrients: NutrientProfile,
    pub meal: MealType,
    pub consumed_at: DateTime<Utc>,
    pub local_date: chrono::NaiveDate,
}
