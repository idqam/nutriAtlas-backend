use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::{
    food::{FoodId, ServingUnit},
    user::UserId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecipeId(pub Uuid);

impl RecipeId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RecipeId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RecipeItemId(pub Uuid);

impl RecipeItemId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for RecipeItemId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FdcFoodType {
    Common,
    Branded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RecipeFoodReference {
    Fdc { fdc_id: i64, food_type: FdcFoodType },
    Custom { food_id: FoodId },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecipeItem {
    pub id: RecipeItemId,
    pub position: u32,
    pub food: RecipeFoodReference,
    pub food_name: String,
    pub quantity: f64,
    pub unit: ServingUnit,
    pub grams: f64,
}

impl RecipeItem {
    pub fn validate(&self) -> Result<(), RecipeValidationError> {
        if self.food_name.trim().is_empty() {
            return Err(RecipeValidationError::EmptyItemName);
        }
        if !self.quantity.is_finite() || self.quantity <= 0.0 {
            return Err(RecipeValidationError::InvalidQuantity(self.quantity));
        }
        if !self.grams.is_finite() || self.grams <= 0.0 {
            return Err(RecipeValidationError::InvalidGrams(self.grams));
        }
        if matches!(self.food, RecipeFoodReference::Fdc { fdc_id, .. } if fdc_id <= 0) {
            return Err(RecipeValidationError::InvalidFdcId);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Recipe {
    pub id: RecipeId,
    pub owner_user_id: UserId,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub total_servings: f64,
    pub items: Vec<RecipeItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Recipe {
    pub fn validate(&self) -> Result<(), RecipeValidationError> {
        if self.name.trim().is_empty() {
            return Err(RecipeValidationError::EmptyName);
        }
        if !self.total_servings.is_finite() || self.total_servings <= 0.0 {
            return Err(RecipeValidationError::InvalidServings(self.total_servings));
        }
        if self.items.is_empty() {
            return Err(RecipeValidationError::NoItems);
        }
        let mut positions = self
            .items
            .iter()
            .map(|item| item.position)
            .collect::<Vec<_>>();
        positions.sort_unstable();
        if positions.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(RecipeValidationError::DuplicateItemPosition);
        }
        for item in &self.items {
            item.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum RecipeValidationError {
    EmptyName,
    EmptyItemName,
    InvalidServings(f64),
    InvalidQuantity(f64),
    InvalidGrams(f64),
    InvalidFdcId,
    NoItems,
    DuplicateItemPosition,
}

impl std::fmt::Display for RecipeValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "invalid recipe: {self:?}")
    }
}

impl std::error::Error for RecipeValidationError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_recipe_item_order_and_amounts() {
        let user_id = UserId::new();
        let recipe = Recipe {
            id: RecipeId::new(),
            owner_user_id: user_id,
            name: "Oatmeal".into(),
            description: None,
            instructions: None,
            total_servings: 1.0,
            items: vec![RecipeItem {
                id: RecipeItemId::new(),
                position: 0,
                food: RecipeFoodReference::Fdc {
                    fdc_id: 123,
                    food_type: FdcFoodType::Common,
                },
                food_name: "Oats".into(),
                quantity: 50.0,
                unit: ServingUnit::Gram,
                grams: 50.0,
            }],
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert!(recipe.validate().is_ok());
    }
}
