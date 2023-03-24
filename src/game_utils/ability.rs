use super::traits::CombatStats;



// Enum describing all possible combat abilities
pub enum Ability {

}

impl Ability {
  pub fn get_stats(&self) -> (CombatStats, CombatStats) {
    match self {
      _ => (CombatStats::new(), CombatStats::new()),
    }
  }
}