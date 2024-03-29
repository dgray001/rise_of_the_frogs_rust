use crate::numeric::random_int;

use super::environment::Position;
use super::ability::Ability;


// Trait for position
pub trait Positionable {
  fn randomize_position(&mut self) {
    match random_int(1, 3) {
      1 => self.set_position(Position::NEAR),
      2 => self.set_position(Position::MEDIUM),
      _ => self.set_position(Position::FAR),
    }
  }

  fn position(&self) -> Position;
  fn set_position(&mut self, position: Position);
}


// Train for damage
pub trait Damageable {
  fn damage_percent(&mut self, amount: f64, max_health: bool) {
    if max_health {
      self.damage(amount * self.max_health());
    }
    else {
      self.damage(amount * self.health());
    }
  }

  fn damage(&mut self, amount: f64);
  fn heal(&mut self, amount: f64);
  fn health(&self) -> f64;
  fn max_health(&self) -> f64;
  fn defense(&self) -> f64;
  fn resistance(&self) -> f64;
}


// Struct to hold combat stats
pub struct CombatStats {
  attack: f64,
  magic: f64,
  piercing: f64,
  penetration: f64,
}

impl CombatStats {
  pub fn new() -> CombatStats {
    return CombatStats {
      attack: 0.0,
      magic: 0.0,
      piercing: 0.0,
      penetration: 0.0,
    }
  }
}


// Enum describing the types of damage
pub enum DamageType {
  PHYSICAL,
  MAGICAL,
  MIXED,
  TRUE,
}


// Trait for combat
pub trait Combatable : Damageable {
  fn use_ability(&mut self, ability: Ability, target: &mut dyn Damageable) {
    // get attacker's stats
    let (ability_extras, ability_factors, damage_type) = ability.get_stats();
    let attack = (ability_extras.attack + self.attack()) * ability_factors.attack;
    let magic = (ability_extras.magic + self.magic()) * ability_factors.magic;
    let piercing = (ability_extras.piercing + self.piercing()) * ability_factors.piercing;
    let penetration = (ability_extras.penetration + self.penetration()) * ability_factors.penetration;
    // get defender's stats
    let defense = target.defense();
    let resistance = target.resistance();
    // calculate effective defense
    let mut effective_defense = 0.0;
    match damage_type {
      DamageType::PHYSICAL => {
        effective_defense = (1.0 - piercing) * defense;
      },
      DamageType::MAGICAL => {
        effective_defense = (1.0 - penetration) * resistance;
      },
      DamageType::MIXED => {
        effective_defense = (1.0 - piercing) * defense + (1.0 - penetration) * resistance;
      },
      DamageType::TRUE => {},
    }
    // calculate damage
    let mut damage = attack + magic - effective_defense;
    if damage < ability.minimum_damage() {
      damage = ability.minimum_damage();
    }
    // apply damage
    target.damage(damage);
  }

  fn attack(&self) -> f64;
  fn magic(&self) -> f64;
  fn piercing(&self) -> f64;
  fn penetration(&self) -> f64;
}