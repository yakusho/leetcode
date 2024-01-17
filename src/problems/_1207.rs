use std::collections::{BTreeMap, HashSet};

struct Solution {}

impl Solution {
  pub fn unique_occurrences(arr: Vec<i32>) -> bool {
    let mut btree_map = BTreeMap::new();
    let mut seen_values = HashSet::new();

    for num in arr.iter() {
      *btree_map.entry(num).or_insert(0) += 1;
    }

    for (_, value) in btree_map {
        if !seen_values.insert(value) {
            return false;
        }
    }

    return true;
  }
}

#[test]
fn test() {
    let t1 = vec![1,2,2,1,1,3];
    assert_eq!(
        Solution::unique_occurrences(t1),
        true
    );
  
    let t2 = vec![1,2];
    assert_eq!(
        Solution::unique_occurrences(t2),
        false
    );

    let t3 = vec![-3,0,1,-3,1,1,1,-3,10,0];
    assert_eq!(
      Solution::unique_occurrences(t3),
      true
  );
}
