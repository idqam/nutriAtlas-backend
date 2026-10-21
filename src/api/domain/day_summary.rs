use std::collections::HashMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use super::{
    food::{MacroNutrients, Nutrient, NutrientTotal},
    meal::FoodLogEntry,
    user::UserId,
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaySummary {
    pub user_id: UserId,
    pub local_date: NaiveDate,
    pub macros: MacroNutrients,
    pub nutrients: HashMap<Nutrient, NutrientTotal>,
    pub entry_count: usize,
}

impl DaySummary {
    pub fn from_entries<'a>(
        user_id: UserId,
        local_date: NaiveDate,
        entries: impl IntoIterator<Item = &'a FoodLogEntry>,
    ) -> Self {
        let entries: Vec<_> = entries
            .into_iter()
            .filter(|entry| entry.user_id == user_id && entry.local_date == local_date)
            .collect();

        let macros = entries
            .iter()
            .fold(MacroNutrients::default(), |total, entry| {
                total.add(&entry.nutrients.macros)
            });
        let total_kcal = macros.calories_kcal;
        let mut nutrients = Nutrient::ALL
            .into_iter()
            .map(|nutrient| {
                (
                    nutrient,
                    NutrientTotal {
                        total_kcal,
                        ..NutrientTotal::default()
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        for entry in &entries {
            let entry_kcal = entry.nutrients.macros.calories_kcal;
            for (nutrient, value) in entry.nutrients.micros.iter() {
                if let Some(value) = value {
                    let total = nutrients
                        .get_mut(&nutrient)
                        .expect("every nutrient is initialized in the summary");
                    total.logged += value;
                    total.covered_kcal += entry_kcal;
                }
            }
        }

        Self {
            user_id,
            local_date,
            macros,
            nutrients,
            entry_count: entries.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;
    use crate::api::domain::{
        food::{FoodId, FoodSource, MicroNutrients, NutrientProfile},
        meal::{FoodLogEntry, LogEntryId, MealType},
    };

    fn entry(
        user_id: UserId,
        local_date: NaiveDate,
        calories: f64,
        fiber: Option<f64>,
    ) -> FoodLogEntry {
        let micros = MicroNutrients {
            fiber_g: fiber,
            ..MicroNutrients::default()
        };
        FoodLogEntry {
            id: LogEntryId::new(),
            user_id,
            food_id: FoodId::new(),
            food_name: "Test food".into(),
            source: FoodSource::Common,
            unit: super::super::food::ServingUnit::Gram,
            quantity: 100.0,
            grams: 100.0,
            nutrients: NutrientProfile {
                macros: MacroNutrients {
                    calories_kcal: calories,
                    ..MacroNutrients::default()
                },
                micros,
            },
            meal: MealType::Lunch,
            consumed_at: Utc.with_ymd_and_hms(2026, 1, 1, 12, 0, 0).unwrap(),
            local_date,
        }
    }

    #[test]
    fn summary_counts_only_matching_user_and_local_date() {
        let user_id = UserId::new();
        let date = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
        let other_date = NaiveDate::from_ymd_opt(2026, 1, 2).unwrap();
        let entries = [
            entry(user_id, date, 200.0, Some(3.0)),
            entry(user_id, date, 300.0, None),
            entry(user_id, other_date, 500.0, Some(5.0)),
            entry(UserId::new(), date, 700.0, Some(7.0)),
        ];

        let summary = DaySummary::from_entries(user_id, date, &entries);
        assert_eq!(summary.entry_count, 2);
        assert_eq!(summary.macros.calories_kcal, 500.0);
        let fiber = summary.nutrients.get(&Nutrient::Fiber).unwrap();
        assert_eq!(fiber.logged, 3.0);
        assert_eq!(fiber.covered_kcal, 200.0);
        assert_eq!(fiber.total_kcal, 500.0);
    }
}
