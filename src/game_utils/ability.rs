use std::fmt;
use std::str::FromStr;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use super::traits::CombatStats;
use super::traits::DamageType;


// Enum describing all possible combat abilities
#[derive(Debug, EnumIter, Eq, Hash, PartialEq, Clone)]
pub enum Ability {
  NOTHING,
}

impl fmt::Display for Ability {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl FromStr for Ability {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    for state in Ability::iter() {
      if state.to_string() == s {
        return Ok(state);
      }
    }
    Err(())
  }
}

impl Ability {
  pub fn get_stats(&self) -> (CombatStats, CombatStats, DamageType) {
    match self {
      _ => (CombatStats::new(), CombatStats::new(), DamageType::MIXED),
    }
  }

  pub fn minimum_damage(&self) -> f64 {
    match self {
      _ => 0.0,
    }
  }
}