use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FoodId(pub Uuid);

impl FoodId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for FoodId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServingUnit {
    Gram,
    Milligram,
    Kilogram,
    Lb,
    Ounce,
    Milliliter,
    Liter,
    Cup,
    Teaspoon,
    Tablespoon,
    FluidOunce,
    Pint,
    Quart,
    Gallon,
    Piece,
    Each,
    Serving,
    Slice,
    Package,
    Packet,
    Box,
    Bag,
    Bottle,
    Can,
    Carton,
    Container,
    Jar,
    Bowl,
    Scoop,
    Stick,
    Bunch,
    Clove,
    Egg,
    Loaf,
}

impl ServingUnit {
    pub fn grams_if_mass(self) -> Option<f64> {
        match self {
            ServingUnit::Gram => Some(1.0),
            ServingUnit::Milligram => Some(0.001),
            ServingUnit::Kilogram => Some(1000.0),
            ServingUnit::Lb => Some(453.592_37),
            ServingUnit::Ounce => Some(28.349_523_125),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ServingSize {
    pub unit: ServingUnit,
    pub quantity: f64,
    pub grams: f64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FoodSource {
    Common,   // USDA / NCCDB style, full panel
    Branded,  // label data, mandatory nutrients only
    Custom,   // user-created
    QuickAdd, // calories and macros only
}

impl FoodSource {
    pub fn micros_usually_incomplete(self) -> bool {
        !matches!(self, FoodSource::Common)
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct MacroNutrients {
    pub calories_kcal: f64,
    pub protein_g: f64,
    pub carbs_g: f64,
    pub fat_g: f64,
}

impl MacroNutrients {
    pub fn scaled(&self, factor: f64) -> Self {
        Self {
            calories_kcal: self.calories_kcal * factor,
            protein_g: self.protein_g * factor,
            carbs_g: self.carbs_g * factor,
            fat_g: self.fat_g * factor,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            calories_kcal: self.calories_kcal + other.calories_kcal,
            protein_g: self.protein_g + other.protein_g,
            carbs_g: self.carbs_g + other.carbs_g,
            fat_g: self.fat_g + other.fat_g,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NutrientUnit {
    Gram,
    Milligram,
    Microgram,
    InternationalUnit,
}

impl NutrientUnit {
    pub fn symbol(self) -> &'static str {
        match self {
            NutrientUnit::Gram => "g",
            NutrientUnit::Milligram => "mg",
            NutrientUnit::Microgram => "mcg",
            NutrientUnit::InternationalUnit => "IU",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Nutrient {
    Fiber,
    Sugar,
    Sodium,
    Cholesterol,
    VitaminA,
    VitaminC,
    VitaminD,
    VitaminE,
    VitaminK,
    Thiamin,
    Riboflavin,
    Niacin,
    PantothenicAcid,
    VitaminB6,
    Folate,
    VitaminB12,
    Calcium,
    Iron,
    Magnesium,
    Phosphorus,
    Potassium,
    Zinc,
    Copper,
    Manganese,
    Selenium,
    Iodine,
    Choline,
}

impl Nutrient {
    pub const ALL: [Nutrient; 27] = [
        Nutrient::Fiber,
        Nutrient::Sugar,
        Nutrient::Sodium,
        Nutrient::Cholesterol,
        Nutrient::VitaminA,
        Nutrient::VitaminC,
        Nutrient::VitaminD,
        Nutrient::VitaminE,
        Nutrient::VitaminK,
        Nutrient::Thiamin,
        Nutrient::Riboflavin,
        Nutrient::Niacin,
        Nutrient::PantothenicAcid,
        Nutrient::VitaminB6,
        Nutrient::Folate,
        Nutrient::VitaminB12,
        Nutrient::Calcium,
        Nutrient::Iron,
        Nutrient::Magnesium,
        Nutrient::Phosphorus,
        Nutrient::Potassium,
        Nutrient::Zinc,
        Nutrient::Copper,
        Nutrient::Manganese,
        Nutrient::Selenium,
        Nutrient::Iodine,
        Nutrient::Choline,
    ];

    pub fn unit(self) -> NutrientUnit {
        match self {
            Nutrient::Fiber | Nutrient::Sugar => NutrientUnit::Gram,
            Nutrient::VitaminA => NutrientUnit::InternationalUnit,
            Nutrient::VitaminD
            | Nutrient::VitaminK
            | Nutrient::Folate
            | Nutrient::VitaminB12
            | Nutrient::Selenium
            | Nutrient::Iodine => NutrientUnit::Microgram,
            Nutrient::Sodium
            | Nutrient::Cholesterol
            | Nutrient::VitaminC
            | Nutrient::VitaminE
            | Nutrient::Thiamin
            | Nutrient::Riboflavin
            | Nutrient::Niacin
            | Nutrient::PantothenicAcid
            | Nutrient::VitaminB6
            | Nutrient::Calcium
            | Nutrient::Iron
            | Nutrient::Magnesium
            | Nutrient::Phosphorus
            | Nutrient::Potassium
            | Nutrient::Zinc
            | Nutrient::Copper
            | Nutrient::Manganese
            | Nutrient::Choline => NutrientUnit::Milligram,
        }
    }

    pub fn mandatory_on_us_label(self) -> bool {
        matches!(
            self,
            Nutrient::Fiber
                | Nutrient::Sugar
                | Nutrient::Sodium
                | Nutrient::Cholesterol
                | Nutrient::VitaminD
                | Nutrient::Calcium
                | Nutrient::Iron
                | Nutrient::Potassium
        )
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct MicroNutrients {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fiber_g: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sugar_g: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sodium_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cholesterol_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vitamin_a_iu: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vitamin_c_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vitamin_d_mcg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vitamin_e_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vitamin_k_mcg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub thiamin_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub riboflavin_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub niacin_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pantothenic_acid_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vitamin_b6_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub folate_mcg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub vitamin_b12_mcg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calcium_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iron_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub magnesium_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub phosphorus_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub potassium_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub zinc_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub copper_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manganese_mg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selenium_mcg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub iodine_mcg: Option<f64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub choline_mg: Option<f64>,
}

impl MicroNutrients {
    pub fn get(&self, nutrient: Nutrient) -> Option<f64> {
        match nutrient {
            Nutrient::Fiber => self.fiber_g,
            Nutrient::Sugar => self.sugar_g,
            Nutrient::Sodium => self.sodium_mg,
            Nutrient::Cholesterol => self.cholesterol_mg,
            Nutrient::VitaminA => self.vitamin_a_iu,
            Nutrient::VitaminC => self.vitamin_c_mg,
            Nutrient::VitaminD => self.vitamin_d_mcg,
            Nutrient::VitaminE => self.vitamin_e_mg,
            Nutrient::VitaminK => self.vitamin_k_mcg,
            Nutrient::Thiamin => self.thiamin_mg,
            Nutrient::Riboflavin => self.riboflavin_mg,
            Nutrient::Niacin => self.niacin_mg,
            Nutrient::PantothenicAcid => self.pantothenic_acid_mg,
            Nutrient::VitaminB6 => self.vitamin_b6_mg,
            Nutrient::Folate => self.folate_mcg,
            Nutrient::VitaminB12 => self.vitamin_b12_mcg,
            Nutrient::Calcium => self.calcium_mg,
            Nutrient::Iron => self.iron_mg,
            Nutrient::Magnesium => self.magnesium_mg,
            Nutrient::Phosphorus => self.phosphorus_mg,
            Nutrient::Potassium => self.potassium_mg,
            Nutrient::Zinc => self.zinc_mg,
            Nutrient::Copper => self.copper_mg,
            Nutrient::Manganese => self.manganese_mg,
            Nutrient::Selenium => self.selenium_mcg,
            Nutrient::Iodine => self.iodine_mcg,
            Nutrient::Choline => self.choline_mg,
        }
    }

    pub fn set(&mut self, nutrient: Nutrient, value: Option<f64>) {
        match nutrient {
            Nutrient::Fiber => self.fiber_g = value,
            Nutrient::Sugar => self.sugar_g = value,
            Nutrient::Sodium => self.sodium_mg = value,
            Nutrient::Cholesterol => self.cholesterol_mg = value,
            Nutrient::VitaminA => self.vitamin_a_iu = value,
            Nutrient::VitaminC => self.vitamin_c_mg = value,
            Nutrient::VitaminD => self.vitamin_d_mcg = value,
            Nutrient::VitaminE => self.vitamin_e_mg = value,
            Nutrient::VitaminK => self.vitamin_k_mcg = value,
            Nutrient::Thiamin => self.thiamin_mg = value,
            Nutrient::Riboflavin => self.riboflavin_mg = value,
            Nutrient::Niacin => self.niacin_mg = value,
            Nutrient::PantothenicAcid => self.pantothenic_acid_mg = value,
            Nutrient::VitaminB6 => self.vitamin_b6_mg = value,
            Nutrient::Folate => self.folate_mcg = value,
            Nutrient::VitaminB12 => self.vitamin_b12_mcg = value,
            Nutrient::Calcium => self.calcium_mg = value,
            Nutrient::Iron => self.iron_mg = value,
            Nutrient::Magnesium => self.magnesium_mg = value,
            Nutrient::Phosphorus => self.phosphorus_mg = value,
            Nutrient::Potassium => self.potassium_mg = value,
            Nutrient::Zinc => self.zinc_mg = value,
            Nutrient::Copper => self.copper_mg = value,
            Nutrient::Manganese => self.manganese_mg = value,
            Nutrient::Selenium => self.selenium_mcg = value,
            Nutrient::Iodine => self.iodine_mcg = value,
            Nutrient::Choline => self.choline_mg = value,
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (Nutrient, Option<f64>)> + '_ {
        Nutrient::ALL.iter().map(move |&n| (n, self.get(n)))
    }

    pub fn scaled(&self, factor: f64) -> Self {
        let mut out = Self::default();
        for (n, v) in self.iter() {
            out.set(n, v.map(|x| x * factor));
        }
        out
    }

    pub fn reported_count(&self) -> usize {
        self.iter().filter(|(_, v)| v.is_some()).count()
    }

    pub fn completeness(&self) -> f64 {
        self.reported_count() as f64 / Nutrient::ALL.len() as f64
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct NutrientProfile {
    pub macros: MacroNutrients,
    pub micros: MicroNutrients,
}

impl NutrientProfile {
    pub fn scaled(&self, factor: f64) -> Self {
        Self {
            macros: self.macros.scaled(factor),
            micros: self.micros.scaled(factor),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Food {
    pub id: FoodId,
    pub name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    pub source: FoodSource,
    pub per_100g: NutrientProfile,
    #[serde(default)]
    pub servings: Vec<ServingSize>,
}

impl Food {
    pub fn grams_for(&self, unit: ServingUnit, quantity: f64) -> Option<f64> {
        if let Some(g) = unit.grams_if_mass() {
            return Some(g * quantity);
        }
        self.servings
            .iter()
            .find(|s| s.unit == unit && s.quantity > 0.0)
            .map(|s| s.grams / s.quantity * quantity)
    }

    pub fn nutrients_for_grams(&self, grams: f64) -> NutrientProfile {
        self.per_100g.scaled(grams / 100.0)
    }

    pub fn nutrients_for(&self, unit: ServingUnit, quantity: f64) -> Option<NutrientProfile> {
        self.grams_for(unit, quantity)
            .map(|g| self.nutrients_for_grams(g))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn chickpeas(source: FoodSource, with_micros: bool) -> Food {
        let mut micros = MicroNutrients::default();
        micros.fiber_g = Some(7.6);
        micros.iron_mg = Some(2.9);
        if with_micros {
            micros.magnesium_mg = Some(48.0);
            micros.vitamin_k_mcg = Some(4.0);
        }
        Food {
            id: FoodId::new(),
            name: "Chickpeas".into(),
            brand: None,
            source,
            per_100g: NutrientProfile {
                macros: MacroNutrients {
                    calories_kcal: 164.0,
                    protein_g: 8.9,
                    carbs_g: 27.4,
                    fat_g: 2.6,
                },
                micros,
            },
            servings: vec![ServingSize {
                unit: ServingUnit::Cup,
                quantity: 1.0,
                grams: 164.0,
            }],
        }
    }

    #[test]
    fn missing_is_none_not_zero() {
        let branded = chickpeas(FoodSource::Branded, false);
        let p = branded.nutrients_for_grams(200.0);
        assert_eq!(p.micros.magnesium_mg, None);
        assert_eq!(p.micros.iron_mg, Some(5.8));
    }

    #[test]
    fn scaling_is_linear() {
        let f = chickpeas(FoodSource::Common, true);
        let p = f.nutrients_for_grams(50.0);
        assert!((p.macros.calories_kcal - 82.0).abs() < 1e-9);
        assert!((p.micros.magnesium_mg.unwrap() - 24.0).abs() < 1e-9);
    }

    #[test]
    fn unit_conversion() {
        let f = chickpeas(FoodSource::Common, true);
        assert_eq!(f.grams_for(ServingUnit::Gram, 10.0), Some(10.0));
        assert_eq!(f.grams_for(ServingUnit::Cup, 0.5), Some(82.0));
        assert_eq!(f.grams_for(ServingUnit::Slice, 1.0), None);
    }

    #[test]
    fn nutrient_table_is_consistent() {
        let mut m = MicroNutrients::default();
        for &n in Nutrient::ALL.iter() {
            m.set(n, Some(1.0));
        }
        assert_eq!(m.reported_count(), Nutrient::ALL.len());
        assert!((m.completeness() - 1.0).abs() < 1e-9);
        for &n in Nutrient::ALL.iter() {
            assert_eq!(m.get(n), Some(1.0));
        }
    }

    #[test]
    fn serde_skips_missing_micros() {
        let f = chickpeas(FoodSource::Branded, false);
        let json = serde_json::to_string(&f).unwrap();
        assert!(!json.contains("magnesium_mg"));
        assert!(json.contains("fiber_g"));
        let back: Food = serde_json::from_str(&json).unwrap();
        assert_eq!(back, f);
    }
}