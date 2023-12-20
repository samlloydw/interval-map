#[cfg(test)]

use super::{IntervalMap, BTreeMap};

#[test]
fn test_empty_indexing() {
    let map = IntervalMap::new(3);
    assert_eq!(map[&0], 3);
    assert_eq!(map[&i32::MAX], 3);
    assert_eq!(map[&i32::MIN], 3);
}

#[test]
fn test_indexing() {
    let mut map: IntervalMap<i32, i32> = IntervalMap::new(3);
    map.value_map = BTreeMap::from([(2,5),(4,6),(10,3)]);
    assert_eq!(map[&-1], 3);
    assert_eq!(map[&i32::MIN], 3);
    assert_eq!(map[&0], 3);
    assert_eq!(map[&2], 5);
    assert_eq!(map[&3], 5);
    assert_eq!(map[&4], 6);
    assert_eq!(map[&9], 6);
    assert_eq!(map[&10], 3);
    assert_eq!(map[&i32::MAX], 3);
}

#[test]
fn test_default() {
    let mut map: IntervalMap<i32, char> = IntervalMap::new('a');
    map.assign(&1, &10, &'c');
    assert_eq!(map[&0], 'a'); // test start point
    assert_eq!(map[&1], 'c');
    assert_eq!(map[&9], 'c');
    assert_eq!(map[&10], 'a');
}

#[test]
fn test_overwrite() {
    let mut map: IntervalMap<i32, char> = IntervalMap::new('a');
    map.assign(&2, &5, &'b'); // general assign
    assert_eq!(map[&1], 'a'); // test start point
    assert_eq!(map[&2], 'b');
    assert_eq!(map[&4], 'b');
    assert_eq!(map[&5], 'a'); // test end point
    map.assign(&2, &5, &'c'); // same range
    assert_eq!(map[&2], 'c');
    assert_eq!(map[&4], 'c');
    assert_eq!(map[&5], 'a'); // test endpoint
    map.assign(&5, &6, &'h'); // same range
    map.assign(&2, &5, &'e'); // same range
    assert_eq!(map[&2], 'e');
    assert_eq!(map[&4], 'e');
    assert_eq!(map[&5], 'h'); // ensure not replaced at key_end
    assert_eq!(map[&6], 'a'); // check end is default
    assert_eq!(map.value_map.keys().count(), 3); // sucessfully added keys
    map.assign(&0, &10, &'d'); // different overwrite range
    assert_eq!(map[&0], 'd');
    assert_eq!(map[&2], 'd');
    assert_eq!(map[&5], 'd');
    assert_eq!(map[&9], 'd');
    assert_eq!(map[&10], 'a');
    assert_eq!(map.value_map.keys().count(), 2); // sucessfully removed keys.
}

#[test]
fn test_canonical() {
    let mut map: IntervalMap<i32, char> = IntervalMap::new('a');
    map.assign(&0, &1, &'a');
    assert_eq!(map.value_map.keys().count(), 0);
    map.assign(&0, &1, &'b');
    assert_eq!(map.value_map.keys().count(), 2);
    map.assign(&1, &2, &'b'); 
    map.assign(&-1, &0, &'b'); 
    assert_eq!(map.value_map.keys().count(), 2);
}

#[test]
fn test_default_intervals() {
    let mut map: IntervalMap<i32, char> = IntervalMap::new('a');
    map.assign(&0, &6, &'b');
    map.assign(&10, &21, &'b');
    assert_eq!(map[&-1], 'a');
    assert_eq!(map[&0], 'b');
    assert_eq!(map[&6], 'a');
    assert_eq!(map[&9], 'a');
    assert_eq!(map[&10], 'b');
    assert_eq!(map[&21], 'a');
}

#[test]
fn test_wrong_inputs() {
    let mut map: IntervalMap<i32, char> = IntervalMap::new('a');
    assert!(!map.assign(&1, &1, &'b')); // test equal
    assert!(!map.assign(&1, &-1, &'c')); // test end < start
}

/// Test the max and min key value for i32 so next key is still there
#[test]
fn test_limits() {
    let mut map: IntervalMap<i32, char> = IntervalMap::new('a');
    assert!(map.assign(&(i32::MAX-1), &i32::MAX, &'b'));
    assert!(map.assign(&i32::MIN, &(i32::MIN+1), &'b'));
    assert!(map.assign(&-1, &0, &'b'));
    assert_eq!(map.value_map.keys().count(), 6);
}

/// Test that the next key isn't the same.
#[test]
fn test_next() {
    let mut map: IntervalMap<i32, char> = IntervalMap::new('a');
    map.assign(&0, &10, &'b');
    assert!(!map.assign(&3, &10, &'b'));
    assert!(!map.assign(&0, &7, &'b'));
    assert!(map.assign(&0, &7, &'c'));
}
