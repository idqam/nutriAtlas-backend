use chrono::NaiveDate;
use chrono_tz::Tz;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::targets::DailyTargets;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct UserId(pub Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Sex {
    Female,
    Male,
    Intersex,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ActivityLevel {
    Sedentary,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtremelyActive,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    #[default]
    User,
    Coach,
    Admin,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Plan {
    #[default]
    Free,
    Premium,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize)]
pub struct BodyMetrics {
    pub height_cm: Option<f64>,
    pub current_weight_kg: Option<f64>,
    pub activity_level: Option<ActivityLevel>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Profile {
    pub sex: Option<Sex>,
    pub birth_date: Option<NaiveDate>,
    pub body_metrics: BodyMetrics,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Goal {
    LoseWeight,
    BetterHealth,
    GainWeight,
    #[default]
    NonSpecified,
}

impl Goal {
    pub fn conflicts_with(self, other: Goal) -> bool {
        matches!(
            (self, other),
            (Goal::LoseWeight, Goal::GainWeight) | (Goal::GainWeight, Goal::LoseWeight)
        )
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub user_id: UserId,
    pub role: Role,
    pub plan: Plan,
    pub timezone: Tz,
    pub pref_units: String,
    pub goals: Vec<Goal>,
    #[serde(default)]
    pub profile: Option<Profile>,
    #[serde(default)]
    pub daily_targets: Vec<DailyTargets>,
}

impl User {
    pub fn new(
        user_id: UserId,
        role: Role,
        plan: Plan,
        timezone: String,
        pref_units: String,
        goals: Vec<Goal>,
    ) -> Result<Self, String> {
        let goals = Self::normalize(goals);
        Self::validate_goals(&goals)?;
        Ok(Self {
            user_id,
            role,
            plan,
            timezone: timezone
                .parse::<Tz>()
                .map_err(|_| format!("Invalid timezone: {timezone}"))?,
            pref_units,
            goals,
            profile: None,
            daily_targets: Vec::new(),
        })
    }

    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }

    pub fn set_goals(&mut self, goals: Vec<Goal>) -> Result<(), String> {
        let goals = Self::normalize(goals);
        Self::validate_goals(&goals)?;
        self.goals = goals;
        Ok(())
    }

    pub fn add_goal(&mut self, goal: Goal) -> Result<(), String> {
        if goal == Goal::NonSpecified || self.goals.contains(&goal) {
            return Ok(());
        }
        if let Some(existing) = self.goals.iter().find(|g| g.conflicts_with(goal)) {
            return Err(format!("{goal:?} conflicts with {existing:?}"));
        }
        self.goals.retain(|g| *g != Goal::NonSpecified);
        self.goals.push(goal);
        Ok(())
    }

    pub fn remove_goal(&mut self, goal: Goal) {
        self.goals.retain(|g| *g != goal);
        if self.goals.is_empty() {
            self.goals.push(Goal::NonSpecified);
        }
    }

    pub fn set_daily_targets(&mut self, targets: DailyTargets) -> Result<(), String> {
        targets.validate().map_err(|error| error.to_string())?;
        if let Some(existing) = self
            .daily_targets
            .iter_mut()
            .find(|existing| existing.effective_from == targets.effective_from)
        {
            *existing = targets;
        } else {
            self.daily_targets.push(targets);
        }
        self.daily_targets
            .sort_by_key(|targets| targets.effective_from);
        Ok(())
    }

    pub fn targets_for(&self, date: NaiveDate) -> Option<&DailyTargets> {
        self.daily_targets
            .iter()
            .filter(|targets| targets.effective_from <= date)
            .max_by_key(|targets| targets.effective_from)
    }

    fn validate_goals(goals: &[Goal]) -> Result<(), String> {
        for (i, a) in goals.iter().enumerate() {
            for b in &goals[i + 1..] {
                if a.conflicts_with(*b) {
                    return Err(format!("{a:?} conflicts with {b:?}"));
                }
            }
        }
        Ok(())
    }

    fn normalize(goals: Vec<Goal>) -> Vec<Goal> {
        let mut out: Vec<Goal> = Vec::new();
        for g in goals {
            if g != Goal::NonSpecified && !out.contains(&g) {
                out.push(g);
            }
        }
        if out.is_empty() {
            out.push(Goal::NonSpecified);
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn user() -> User {
        User::new(
            UserId::new(),
            Role::User,
            Plan::Free,
            "America/Bogota".into(),
            "metric".into(),
            vec![],
        )
        .unwrap()
    }

    #[test]
    fn constructor_preserves_external_identity_and_role() {
        let user_id = UserId::new();
        let user = User::new(
            user_id,
            Role::Admin,
            Plan::Premium,
            "America/Bogota".into(),
            "metric".into(),
            vec![],
        )
        .unwrap();

        assert_eq!(user.user_id, user_id);
        assert!(user.is_admin());
        assert_eq!(user.plan, Plan::Premium);
        assert_eq!(user.timezone, "America/Bogota".parse::<Tz>().unwrap());
    }

    #[test]
    fn constructor_rejects_unknown_timezone() {
        assert!(
            User::new(
                UserId::new(),
                Role::User,
                Plan::Free,
                "Not/A-Timezone".into(),
                "metric".into(),
                vec![],
            )
            .is_err()
        );
    }

    #[test]
    fn allows_compatible_goals() {
        let mut u = user();
        u.add_goal(Goal::LoseWeight).unwrap();
        u.add_goal(Goal::BetterHealth).unwrap();
        assert_eq!(u.goals, vec![Goal::LoseWeight, Goal::BetterHealth]);
    }

    #[test]
    fn rejects_conflicting_pair_without_mutating() {
        let mut u = user();
        u.add_goal(Goal::LoseWeight).unwrap();
        assert!(u.add_goal(Goal::GainWeight).is_err());
        assert_eq!(u.goals, vec![Goal::LoseWeight]);
    }

    #[test]
    fn set_goals_rejects_conflict() {
        let mut u = user();
        assert!(
            u.set_goals(vec![Goal::LoseWeight, Goal::GainWeight])
                .is_err()
        );
        assert_eq!(u.goals, vec![Goal::NonSpecified]);
    }

    #[test]
    fn removing_last_goal_falls_back_to_non_specified() {
        let mut u = user();
        u.add_goal(Goal::BetterHealth).unwrap();
        u.remove_goal(Goal::BetterHealth);
        assert_eq!(u.goals, vec![Goal::NonSpecified]);
    }

    #[test]
    fn targets_are_selected_by_effective_date() {
        let mut u = user();
        let earlier = DailyTargets {
            effective_from: NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
            calories_kcal: 2000.0,
            protein_g: 100.0,
            carbs_g: 250.0,
            fat_g: 60.0,
        };
        let later = DailyTargets {
            effective_from: NaiveDate::from_ymd_opt(2026, 2, 1).unwrap(),
            calories_kcal: 1800.0,
            ..earlier
        };
        u.set_daily_targets(later).unwrap();
        u.set_daily_targets(earlier).unwrap();

        assert_eq!(
            u.targets_for(NaiveDate::from_ymd_opt(2026, 1, 15).unwrap()),
            Some(&earlier)
        );
        assert_eq!(
            u.targets_for(NaiveDate::from_ymd_opt(2026, 2, 15).unwrap()),
            Some(&later)
        );
    }
}
