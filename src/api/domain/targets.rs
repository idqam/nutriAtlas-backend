use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct DailyTargets {
    pub effective_from: NaiveDate,
    pub calories_kcal: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
}

impl DailyTargets {
    pub fn validate(&self) -> Result<(), TargetValidationError> {
        for (name, value) in [
            ("calories_kcal", self.calories_kcal),
            ("protein_g", self.protein_g),
            ("carbs_g", self.carbs_g),
            ("fat_g", self.fat_g),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(TargetValidationError { name, value });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetValidationError {
    pub name: &'static str,
    pub value: f64,
}

impl std::fmt::Display for TargetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} must be finite and non-negative, got {}",
            self.name, self.value
        )
    }
}

impl std::error::Error for TargetValidationError {}
