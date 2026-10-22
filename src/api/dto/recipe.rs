use serde::{Deserialize, Serialize};

use crate::api::domain::{
    food::{FoodId, ServingUnit},
    recipe::{FdcFoodType, Recipe, RecipeFoodReference, RecipeId, RecipeItemId},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecipeFoodReferenceRequest {
    FdcCommon { fdc_id: i64 },
    FdcBranded { fdc_id: i64 },
    Custom { food_id: FoodId },
}

impl RecipeFoodReferenceRequest {
    pub fn into_domain(self) -> Result<RecipeFoodReference, String> {
        match self {
            Self::FdcCommon { fdc_id } if fdc_id > 0 => Ok(RecipeFoodReference::Fdc {
                fdc_id,
                food_type: FdcFoodType::Common,
            }),
            Self::FdcBranded { fdc_id } if fdc_id > 0 => Ok(RecipeFoodReference::Fdc {
                fdc_id,
                food_type: FdcFoodType::Branded,
            }),
            Self::Custom { food_id } => Ok(RecipeFoodReference::Custom { food_id }),
            _ => Err("FDC food IDs must be positive".into()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateRecipeItemRequest {
    pub food: RecipeFoodReferenceRequest,
    pub quantity: f64,
    pub unit: ServingUnit,
    pub position: u32,
}

impl CreateRecipeItemRequest {
    pub fn validate(&self) -> Result<(), String> {
        if !self.quantity.is_finite() || self.quantity <= 0.0 {
            return Err("recipe item quantity must be finite and positive".into());
        }
        self.food.into_domain().map(|_| ())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CreateRecipeRequest {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    pub total_servings: f64,
    pub items: Vec<CreateRecipeItemRequest>,
}

impl CreateRecipeRequest {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("recipe name cannot be empty".into());
        }
        if !self.total_servings.is_finite() || self.total_servings <= 0.0 {
            return Err("total_servings must be finite and positive".into());
        }
        if self.items.is_empty() {
            return Err("a recipe must contain at least one item".into());
        }
        let mut positions = self
            .items
            .iter()
            .map(|item| item.position)
            .collect::<Vec<_>>();
        positions.sort_unstable();
        if positions.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err("recipe item positions must be unique".into());
        }
        for item in &self.items {
            item.validate()?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecipeResponse {
    pub id: RecipeId,
    pub name: String,
    pub description: Option<String>,
    pub instructions: Option<String>,
    pub total_servings: f64,
    pub items: Vec<RecipeItemResponse>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RecipeItemResponse {
    pub id: RecipeItemId,
    pub position: u32,
    pub food: RecipeFoodReference,
    pub food_name: String,
    pub quantity: f64,
    pub unit: ServingUnit,
    pub grams: f64,
}

impl From<&Recipe> for RecipeResponse {
    fn from(recipe: &Recipe) -> Self {
        Self {
            id: recipe.id,
            name: recipe.name.clone(),
            description: recipe.description.clone(),
            instructions: recipe.instructions.clone(),
            total_servings: recipe.total_servings,
            items: recipe
                .items
                .iter()
                .map(|item| RecipeItemResponse {
                    id: item.id,
                    position: item.position,
                    food: item.food,
                    food_name: item.food_name.clone(),
                    quantity: item.quantity,
                    unit: item.unit,
                    grams: item.grams,
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recipe_food_references_keep_id_types_distinct() {
        let fdc = RecipeFoodReferenceRequest::FdcBranded { fdc_id: 123 };
        assert_eq!(
            fdc.into_domain().unwrap(),
            RecipeFoodReference::Fdc {
                fdc_id: 123,
                food_type: FdcFoodType::Branded,
            }
        );

        let food_id = FoodId::new();
        assert_eq!(
            RecipeFoodReferenceRequest::Custom { food_id }
                .into_domain()
                .unwrap(),
            RecipeFoodReference::Custom { food_id }
        );
    }

    #[test]
    fn recipe_request_rejects_duplicate_item_positions() {
        let item = CreateRecipeItemRequest {
            food: RecipeFoodReferenceRequest::FdcCommon { fdc_id: 123 },
            quantity: 1.0,
            unit: ServingUnit::Gram,
            position: 0,
        };
        let request = CreateRecipeRequest {
            name: "Oats".into(),
            description: None,
            instructions: None,
            total_servings: 1.0,
            items: vec![item.clone(), item],
        };

        assert!(request.validate().is_err());
    }
}
