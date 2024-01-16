use std::collections::HashMap;
use rand::seq::SliceRandom;

struct RandomizedSet {
    set: HashMap<i32, usize>,
    vec: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RandomizedSet {
    fn new() -> Self {
        RandomizedSet {
            set: HashMap::new(),
            vec: vec![],
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.set.contains_key(&val) {
            false
        } else {
            self.set.insert(val, self.set.len());
            self.vec.push(val);
            true
        }
    }

    fn remove(&mut self, val: i32) -> bool {
        match self.set.get(&val) {
            Some(&index) => {
                self.set.remove(&val);
                self.vec.swap_remove(index);
                
                let last = self.vec.get(index);
                if let Some(&val) = last {
                  self.set.insert(val, index);
                }

                true
            }
            None => false,
        }
    }

    fn get_random(&self) -> i32 {
        return *self.vec.choose(&mut rand::thread_rng()).unwrap();
    }
}
