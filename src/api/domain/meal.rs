use chrono::{DateTime, NaiveDate, Utc};
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    food::{Food, FoodId, FoodSource, NutrientProfile, ServingUnit},
    user::UserId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LogEntryId(pub Uuid);

impl LogEntryId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for LogEntryId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MealType {
    Breakfast,
    Lunch,
    Dinner,
    Snack,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoodLogEntry {
    pub id: LogEntryId,
    pub user_id: UserId,
    pub food_id: FoodId,
    pub food_name: String,
    pub source: FoodSource,
    pub unit: ServingUnit,
    pub quantity: f64,
    pub grams: f64,
    pub nutrients: NutrientProfile,
    pub meal: MealType,
    pub consumed_at: DateTime<Utc>,
    pub local_date: NaiveDate,
}

impl FoodLogEntry {
    pub fn from_food(
        user_id: UserId,
        food: &Food,
        unit: ServingUnit,
        quantity: f64,
        meal: MealType,
        consumed_at: DateTime<Utc>,
        timezone: Tz,
    ) -> Result<Self, FoodLogEntryError> {
        if !quantity.is_finite() || quantity <= 0.0 {
            return Err(FoodLogEntryError::InvalidQuantity(quantity));
        }

        let grams = food
            .grams_for(unit, quantity)
            .filter(|grams| grams.is_finite() && *grams > 0.0)
            .ok_or(FoodLogEntryError::UnknownServingUnit(unit))?;
        Ok(Self {
            id: LogEntryId::new(),
            user_id,
            food_id: food.id,
            food_name: food.name.clone(),
            source: food.source,
            unit,
            quantity,
            grams,
            nutrients: food.nutrients_for_grams(grams),
            meal,
            consumed_at,
            local_date: consumed_at.with_timezone(&timezone).date_naive(),
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FoodLogEntryError {
    InvalidQuantity(f64),
    UnknownServingUnit(ServingUnit),
}

impl std::fmt::Display for FoodLogEntryError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FoodLogEntryError::InvalidQuantity(quantity) => {
                write!(f, "quantity must be finite and positive, got {quantity}")
            }
            FoodLogEntryError::UnknownServingUnit(unit) => {
                write!(f, "no gram conversion is available for {unit:?}")
            }
        }
    }
}

impl std::error::Error for FoodLogEntryError {}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, TimeZone, Utc};

    use super::*;
    use crate::api::domain::food::{MacroNutrients, NutrientProfile};

    #[test]
    fn local_date_uses_the_supplied_timezone() {
        let food = Food {
            id: FoodId::new(),
            name: "Test food".into(),
            brand: None,
            owner: None,
            source: FoodSource::Common,
            per_100g: NutrientProfile {
                macros: MacroNutrients {
                    calories_kcal: 100.0,
                    ..MacroNutrients::default()
                },
                ..NutrientProfile::default()
            },
            servings: Vec::new(),
        };

        let entry = FoodLogEntry::from_food(
            UserId::new(),
            &food,
            ServingUnit::Gram,
            100.0,
            MealType::Dinner,
            Utc.with_ymd_and_hms(2026, 1, 2, 2, 0, 0).unwrap(),
            "America/Bogota".parse().unwrap(),
        )
        .unwrap();

        assert_eq!(
            entry.local_date,
            NaiveDate::from_ymd_opt(2026, 1, 1).unwrap()
        );
    }
}
