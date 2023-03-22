use rand::Rng;


pub fn random_chance(x: f64) -> bool {
  let mut rng = rand::thread_rng();
  return rng.gen::<f64>() < x;
}

// inclusive upper bound
pub fn random_int<T>(min: T, max: T) -> T where
  T: std::cmp::PartialOrd + rand::distributions::uniform::SampleUniform
{
  if max < min {
    return random_int(max, min);
  }
  return rand::thread_rng().gen_range(min..=max);
}


// Struct representing an integer range
pub struct IntegerRange {
  start: i64,
  end: i64,
}

impl IntegerRange {
  pub fn new() -> IntegerRange {
    return IntegerRange {
      start: -1,
      end: -1,
    }
  }

  pub fn from_str(s: &str) -> IntegerRange {
    let mut range = IntegerRange::new();
    if s.contains("-") {
      let (start, end) = s.split_once("-").unwrap();
      range.start = start.parse::<i64>().unwrap_or(-1);
      range.end = end.parse::<i64>().unwrap_or(-1);
    }
    else {
      let num = s.parse::<i64>().unwrap_or(-1);
      range.start = num;
      range.end = num;
    }
    return range;
  }

  pub fn contains(&self, x: i64) -> bool {
    return x >= self.start && x <= self.end;
  }

  pub fn min(&self) -> i64 {
    return self.start;
  }

  pub fn max(&self) -> i64 {
    return self.end;
  }
}