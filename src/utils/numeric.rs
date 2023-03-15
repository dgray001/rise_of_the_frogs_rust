use rand::Rng;

pub fn random_chance(x: f64) -> bool {
  let mut rng = rand::thread_rng();
  return rng.gen::<f64>() < x;
}

pub struct IntegerRange {
  start: i64,
  end: i64,
}

impl IntegerRange {
  pub fn new() -> IntegerRange {
    return IntegerRange {
      start: 0,
      end: 0,
    }
  }

  pub fn from_str(s: &str) -> IntegerRange {
    let mut range = IntegerRange::new();
    if s.contains("-") {
      let (start, end) = s.split_once("-").unwrap();
      range.start = start.parse::<i64>().unwrap_or(0);
      range.end = end.parse::<i64>().unwrap_or(0);
    }
    else {
      let num = s.parse::<i64>().unwrap_or(0);
      range.start = num;
      range.end = num;
    }
    return range;
  }

  pub fn contains(&self, x: i64) -> bool {
    return x >= self.start && x <= self.end;
  }
}