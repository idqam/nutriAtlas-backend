use serde::{Deserialize, Serialize};

use crate::api::domain::{
    food::{Food, FoodId, FoodSource, NutrientProfile, ServingSize},
    user::UserId,
};

use super::common::FoodReference;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateCustomFoodRequest {
    pub name: String,
    #[serde(default)]
    pub brand: Option<String>,
    pub nutrient_profile_per_100g: NutrientProfile,
    #[serde(default)]
    pub servings: Vec<ServingSize>,
}

impl CreateCustomFoodRequest {
    pub fn into_food(self, owner: UserId) -> Result<Food, String> {
        if self.name.trim().is_empty() {
            return Err("food name cannot be empty".into());
        }
        if self.name.trim().chars().count() > 200 {
            return Err("food name cannot exceed 200 characters".into());
        }
        if self.servings.iter().any(|serving| {
            !serving.quantity.is_finite()
                || serving.quantity <= 0.0
                || !serving.grams.is_finite()
                || serving.grams <= 0.0
        }) {
            return Err("serving quantity and grams must be finite and positive".into());
        }
        self.nutrient_profile_per_100g
            .micros
            .validate()
            .map_err(|error| error.to_string())?;
        if [
            self.nutrient_profile_per_100g.macros.calories_kcal,
            self.nutrient_profile_per_100g.macros.protein_g,
            self.nutrient_profile_per_100g.macros.carbs_g,
            self.nutrient_profile_per_100g.macros.fat_g,
        ]
        .iter()
        .any(|value| !value.is_finite() || *value < 0.0)
        {
            return Err("macronutrients must be finite and non-negative".into());
        }

        Ok(Food {
            id: FoodId::new(),
            name: self.name.trim().to_owned(),
            brand: self.brand,
            owner: Some(owner),
            source: FoodSource::Custom,
            per_100g: self.nutrient_profile_per_100g,
            servings: self.servings,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FoodResponse {
    pub reference: FoodReference,
    pub name: String,
    pub brand: Option<String>,
    pub source: FoodSource,
    pub nutrient_profile_per_100g: NutrientProfile,
    pub servings: Vec<ServingSize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_food_is_owned_by_the_authenticated_user() {
        let owner = UserId::new();
        let food = CreateCustomFoodRequest {
            name: "  Oats  ".into(),
            brand: None,
            nutrient_profile_per_100g: NutrientProfile::default(),
            servings: Vec::new(),
        }
        .into_food(owner)
        .unwrap();

        assert_eq!(food.name, "Oats");
        assert_eq!(food.owner, Some(owner));
        assert_eq!(food.source, FoodSource::Custom);
    }
}
