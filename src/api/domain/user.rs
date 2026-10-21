use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
    pub admin: bool,
    pub regular: bool,
    pub premium: bool,
    pub timezone: String,
    pub pref_units: String,
    pub goals: Vec<Goal>,
}

impl User {
    pub fn new(
        admin: bool,
        regular: bool,
        premium: bool,
        timezone: String,
        pref_units: String,
        goals: Vec<Goal>,
    ) -> Result<Self, String> {
        let goals = Self::normalize(goals);
        Self::validate_goals(&goals)?;
        Ok(Self {
            admin,
            regular,
            premium,
            timezone,
            pref_units,
            goals,
        })
    }

    pub fn is_admin(&self) -> bool {
        self.admin
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
        User::new(false, true, false, "America/Bogota".into(), "metric".into(), vec![]).unwrap()
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
        assert!(u.set_goals(vec![Goal::LoseWeight, Goal::GainWeight]).is_err());
        assert_eq!(u.goals, vec![Goal::NonSpecified]);
    }

    #[test]
    fn removing_last_goal_falls_back_to_non_specified() {
        let mut u = user();
        u.add_goal(Goal::BetterHealth).unwrap();
        u.remove_goal(Goal::BetterHealth);
        assert_eq!(u.goals, vec![Goal::NonSpecified]);
    }
}