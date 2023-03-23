use std::collections::HashMap;

use super::item::Item;


// Contains infinite items separable by group
pub struct Inventory {
  pub items: HashMap<u64, Item>,
  pub next_item_key: u64, // will never repeat keys
  pub capacity: usize,
}

impl Inventory {
  pub fn new() -> Inventory {
    return Inventory {
      items: HashMap::new(),
      next_item_key: 1,
      capacity: 1,
    }
  }

  pub fn can_pickup(&self) -> bool {
    return self.items.len() < self.capacity;
  }

  pub fn add(&mut self, item: Item) -> Option<Item> {
    if self.can_pickup() {
      self.items.insert(self.next_item_key, item);
      self.next_item_key += 1;
      return None;
    }
    return Some(item);
  }

  pub fn list(&self) -> Vec<&Item> {
    let mut list = vec![];
    for (_, item) in self.items.iter() {
      list.push(item);
    }
    return list;
  }
}
