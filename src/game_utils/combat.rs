use super::UnitIdentifier;


// Struct describing a team
struct CombatTeam {
  name: String,
  members: Vec<UnitIdentifier>,
}


// Struct describing a combat
pub struct RotfCombat {
  teams: Vec<CombatTeam>,
  pub turn: usize,
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

  pub fn add_team(&mut self, name: &str, members: Vec<UnitIdentifier>) {
    self.teams.push(CombatTeam {
      name: name.to_owned(),
      members,
    });
  }
}