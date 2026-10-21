use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::user::UserId;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PreparationStyle {
    Raw,
    Boiled,
    Baked,
    Fried,
    Grilled,
    Roasted,
    Steamed,
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
    MicrogramRae,
    MicrogramDfe,
    InternationalUnit,
}

impl NutrientUnit {
    pub fn symbol(self) -> &'static str {
        match self {
            NutrientUnit::Gram => "g",
            NutrientUnit::Milligram => "mg",
            NutrientUnit::Microgram => "mcg",
            NutrientUnit::MicrogramRae => "mcg RAE",
            NutrientUnit::MicrogramDfe => "mcg DFE",
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
            Nutrient::VitaminA => NutrientUnit::MicrogramRae,
            Nutrient::Folate => NutrientUnit::MicrogramDfe,
            Nutrient::VitaminD
            | Nutrient::VitaminK
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
pub struct NutrientTotal {
    pub logged: f64,
    pub covered_kcal: f64,
    pub total_kcal: f64,
}

impl NutrientTotal {
    pub fn coverage(&self) -> f64 {
        if !self.total_kcal.is_finite()
            || self.total_kcal <= 0.0
            || !self.covered_kcal.is_finite()
            || self.covered_kcal <= 0.0
        {
            return 0.0;
        }

        (self.covered_kcal / self.total_kcal).clamp(0.0, 1.0)
    }

    pub fn confidence(&self, days_logged: usize) -> Confidence {
        confidence(self.coverage(), days_logged)
    }

    pub fn plausible_range(&self, days_logged: usize) -> Option<(f64, f64)> {
        if self.confidence(days_logged) != Confidence::Usable
            || !self.logged.is_finite()
            || self.logged < 0.0
        {
            return None;
        }

        Some((self.logged * 0.75, self.logged * 1.25))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Insufficient,
    Low,
    Usable,
}

pub fn confidence(coverage: f64, days_logged: usize) -> Confidence {
    let coverage = if coverage.is_finite() {
        coverage.clamp(0.0, 1.0)
    } else {
        0.0
    };

    match (coverage, days_logged) {
        (coverage, _) if coverage < 0.5 => Confidence::Insufficient,
        (coverage, days_logged) if coverage < 0.8 || days_logged < 5 => Confidence::Low,
        _ => Confidence::Usable,
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
    pub vitamin_a_rae_mcg: Option<f64>,
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
    pub folate_mcg_dfe: Option<f64>,
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
    pub fn zero() -> Self {
        let mut nutrients = Self::default();
        for nutrient in Nutrient::ALL {
            nutrients.set(nutrient, Some(0.0));
        }
        nutrients
    }

    pub fn get(&self, nutrient: Nutrient) -> Option<f64> {
        match nutrient {
            Nutrient::Fiber => self.fiber_g,
            Nutrient::Sugar => self.sugar_g,
            Nutrient::Sodium => self.sodium_mg,
            Nutrient::Cholesterol => self.cholesterol_mg,
            Nutrient::VitaminA => self.vitamin_a_rae_mcg,
            Nutrient::VitaminC => self.vitamin_c_mg,
            Nutrient::VitaminD => self.vitamin_d_mcg,
            Nutrient::VitaminE => self.vitamin_e_mg,
            Nutrient::VitaminK => self.vitamin_k_mcg,
            Nutrient::Thiamin => self.thiamin_mg,
            Nutrient::Riboflavin => self.riboflavin_mg,
            Nutrient::Niacin => self.niacin_mg,
            Nutrient::PantothenicAcid => self.pantothenic_acid_mg,
            Nutrient::VitaminB6 => self.vitamin_b6_mg,
            Nutrient::Folate => self.folate_mcg_dfe,
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
            Nutrient::VitaminA => self.vitamin_a_rae_mcg = value,
            Nutrient::VitaminC => self.vitamin_c_mg = value,
            Nutrient::VitaminD => self.vitamin_d_mcg = value,
            Nutrient::VitaminE => self.vitamin_e_mg = value,
            Nutrient::VitaminK => self.vitamin_k_mcg = value,
            Nutrient::Thiamin => self.thiamin_mg = value,
            Nutrient::Riboflavin => self.riboflavin_mg = value,
            Nutrient::Niacin => self.niacin_mg = value,
            Nutrient::PantothenicAcid => self.pantothenic_acid_mg = value,
            Nutrient::VitaminB6 => self.vitamin_b6_mg = value,
            Nutrient::Folate => self.folate_mcg_dfe = value,
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

    pub fn add(&self, other: &Self) -> Self {
        let mut total = Self::default();
        for (nutrient, value) in self.iter() {
            total.set(
                nutrient,
                value
                    .zip(other.get(nutrient))
                    .map(|(left, right)| left + right),
            );
        }
        total
    }

    pub fn validate(&self) -> Result<(), NutrientValueError> {
        for (nutrient, value) in self.iter() {
            let Some(value) = value else {
                continue;
            };

            if !value.is_finite() {
                return Err(NutrientValueError::NonFinite { nutrient, value });
            }
            if value < 0.0 {
                return Err(NutrientValueError::Negative { nutrient, value });
            }
        }
        Ok(())
    }

    pub fn reported_count(&self) -> usize {
        self.iter().filter(|(_, v)| v.is_some()).count()
    }

    pub fn completeness(&self) -> f64 {
        self.reported_count() as f64 / Nutrient::ALL.len() as f64
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NutrientValueError {
    NonFinite { nutrient: Nutrient, value: f64 },
    Negative { nutrient: Nutrient, value: f64 },
}

impl std::fmt::Display for NutrientValueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NutrientValueError::NonFinite { nutrient, value } => {
                write!(f, "{nutrient:?} must be finite, got {value}")
            }
            NutrientValueError::Negative { nutrient, value } => {
                write!(f, "{nutrient:?} cannot be negative, got {value}")
            }
        }
    }
}

impl std::error::Error for NutrientValueError {}

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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<UserId>,
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
        let micros = MicroNutrients {
            fiber_g: Some(7.6),
            iron_mg: Some(2.9),
            magnesium_mg: with_micros.then_some(48.0),
            vitamin_k_mcg: with_micros.then_some(4.0),
            ..MicroNutrients::default()
        };
        Food {
            id: FoodId::new(),
            name: "Chickpeas".into(),
            brand: None,
            owner: None,
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
    fn zero_marks_every_nutrient_as_reported_zero() {
        let nutrients = MicroNutrients::zero();
        assert_eq!(nutrients.reported_count(), Nutrient::ALL.len());
        assert!(Nutrient::ALL.iter().all(|&n| nutrients.get(n) == Some(0.0)));
    }

    #[test]
    fn addition_preserves_missing_values() {
        let first = MicroNutrients {
            fiber_g: Some(1.0),
            ..MicroNutrients::default()
        };
        let second = MicroNutrients {
            fiber_g: Some(2.0),
            sugar_g: Some(3.0),
            ..MicroNutrients::default()
        };

        let total = first.add(&second);
        assert_eq!(total.fiber_g, Some(3.0));
        assert_eq!(total.sugar_g, None);
    }

    #[test]
    fn validation_rejects_negative_and_non_finite_values() {
        let mut nutrients = MicroNutrients {
            sodium_mg: Some(-1.0),
            ..MicroNutrients::default()
        };
        assert!(matches!(
            nutrients.validate(),
            Err(NutrientValueError::Negative {
                nutrient: Nutrient::Sodium,
                value: -1.0
            })
        ));

        nutrients.sodium_mg = Some(f64::NAN);
        assert!(matches!(
            nutrients.validate(),
            Err(NutrientValueError::NonFinite {
                nutrient: Nutrient::Sodium,
                value
            }) if value.is_nan()
        ));
    }

    #[test]
    fn nutrient_confidence_and_ranges_respect_coverage() {
        let insufficient = NutrientTotal {
            logged: 100.0,
            covered_kcal: 400.0,
            total_kcal: 1000.0,
        };
        assert_eq!(insufficient.confidence(7), Confidence::Insufficient);
        assert_eq!(insufficient.plausible_range(7), None);

        let low = NutrientTotal {
            covered_kcal: 700.0,
            ..insufficient
        };
        assert_eq!(low.confidence(3), Confidence::Low);
        assert_eq!(low.plausible_range(7), None);

        let usable = NutrientTotal {
            covered_kcal: 900.0,
            ..insufficient
        };
        assert_eq!(usable.confidence(7), Confidence::Usable);
        assert_eq!(usable.plausible_range(7), Some((75.0, 125.0)));
    }

    #[test]
    fn vitamin_a_and_folate_use_standardized_units() {
        assert_eq!(Nutrient::VitaminA.unit().symbol(), "mcg RAE");
        assert_eq!(Nutrient::Folate.unit().symbol(), "mcg DFE");
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
