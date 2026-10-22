use std::collections::BTreeMap;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::api::domain::{
    day_summary::DaySummary,
    food::{MacroNutrients, Nutrient, NutrientTotal},
};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct DaySummaryResponse {
    pub local_date: NaiveDate,
    pub macros: MacroNutrients,
    pub nutrients: BTreeMap<Nutrient, NutrientTotal>,
    pub entry_count: usize,
}

impl From<DaySummary> for DaySummaryResponse {
    fn from(summary: DaySummary) -> Self {
        Self {
            local_date: summary.local_date,
            macros: summary.macros,
            nutrients: summary.nutrients,
            entry_count: summary.entry_count,
        }
    }
}
