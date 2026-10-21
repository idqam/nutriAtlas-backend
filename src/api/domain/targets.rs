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
        if !self.calories_kcal.is_finite() || self.calories_kcal <= 0.0 {
            return Err(TargetValidationError {
                name: "calories_kcal",
                value: self.calories_kcal,
                must_be_positive: true,
            });
        }

        for (name, value) in [
            ("protein_g", self.protein_g),
            ("carbs_g", self.carbs_g),
            ("fat_g", self.fat_g),
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(TargetValidationError {
                    name,
                    value,
                    must_be_positive: false,
                });
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetValidationError {
    pub name: &'static str,
    pub value: f64,
    pub must_be_positive: bool,
}

impl std::fmt::Display for TargetValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let requirement = if self.must_be_positive {
            "finite and positive"
        } else {
            "finite and non-negative"
        };
        write!(f, "{} must be {requirement}, got {}", self.name, self.value)
    }
}

impl std::error::Error for TargetValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    fn targets() -> DailyTargets {
        DailyTargets {
            effective_from: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            calories_kcal: 2000.0,
            protein_g: 100.0,
            carbs_g: 250.0,
            fat_g: 60.0,
        }
    }

    #[test]
    fn calories_must_be_positive() {
        let mut targets = targets();
        targets.calories_kcal = 0.0;
        assert!(targets.validate().is_err());
    }

    #[test]
    fn macro_targets_may_be_zero() {
        let targets = DailyTargets {
            protein_g: 0.0,
            carbs_g: 0.0,
            fat_g: 0.0,
            ..targets()
        };
        assert!(targets.validate().is_ok());
    }
}
