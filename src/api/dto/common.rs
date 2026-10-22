use serde::{Deserialize, Serialize};

use crate::api::domain::food::{FoodId, MacroNutrients};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum FoodReference {
    Fdc {
        fdc_id: i64,
    },
    Custom {
        food_id: FoodId,
    },
    QuickAdd {
        name: String,
        macros: MacroNutrients,
    },
}

impl FoodReference {
    pub fn validate(&self) -> Result<(), String> {
        match self {
            FoodReference::Fdc { fdc_id } if *fdc_id <= 0 => Err("fdc_id must be positive".into()),
            FoodReference::QuickAdd { name, macros } => {
                if name.trim().is_empty() {
                    return Err("quick-add food name cannot be empty".into());
                }
                if [
                    macros.calories_kcal,
                    macros.protein_g,
                    macros.carbs_g,
                    macros.fat_g,
                ]
                .iter()
                .any(|value| !value.is_finite() || *value < 0.0)
                {
                    return Err("quick-add nutrients must be finite and non-negative".into());
                }
                Ok(())
            }
            _ => Ok(()),
        }
    }
}
