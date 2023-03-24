use super::UnitIdentifier;


// Struct describing a team
struct CombatTeam {
  name: String,
  members: Vec<UnitIdentifier>,
}


// Struct describing a combat
pub struct RotfCombat {
  teams: Vec<CombatTeam>,
  turn: usize,
  turn_number: usize,
}

impl RotfCombat {
  pub fn new() -> RotfCombat {
    return RotfCombat {
      teams: vec![],
      turn: 0,
      turn_number: 0,
    }
  }
}