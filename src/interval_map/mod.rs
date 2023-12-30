use std::collections::BTreeMap;
use std::fmt::Debug;
use std::ops::Index;
use std::hash::Hash;

mod test;

/// A hashmap that returns values from ranges (or intervals).
/// The hashmap is initialised with a default value. One can add
/// or remove key-value pairs. Consecutive keys return different values.
pub struct IntervalMap<K, V>
where
    K: Ord + Eq + Hash + Clone,
    V: Eq + Clone,
{
    default_value: V,
    value_map: BTreeMap<K, V>,
}

impl<K, V> IntervalMap<K, V> 
where
    K: Ord + Eq + Hash + Clone,
    V: Eq + Clone, // Make clone as we have to copy default vals
{
    pub fn new(default: V) -> Self {
        Self { default_value: default, value_map: BTreeMap::new() }
    }

    /// Assign values to the hashmap.
    /// 
    /// Returns true if correctly assigned, false if not assigned.
    /// 
    /// Values aren't assigned if the previous or following values are the same.
    /// If key_end is greater or equal to key_begin, values aren't assigned.
    /// If entires already exist in the domain of key_begin to key_end, they are 
    /// overwritten.
    /// Entries that already exist in the domain of key_begin to key_end whose domain
    /// extends past key_end are shifted so their domain starts at key_end.
    /// The default value is used in the intervals between entries.
    pub fn assign(&mut self, key_begin: &K, key_end: &K, value: &V) -> bool {
        if key_begin >= key_end {
            return false;
        }
        // if empty add value if different.
        // if before other keys, insert and add default value to end.
        if self.value_map.is_empty() && value != &self.default_value {
            self.value_map.insert(key_begin.clone(), value.clone());
            self.value_map.insert(key_end.clone(), self.default_value.clone());
            return true;
        }

        // Consecutive values must be heterogenous (canonical).
        if value == self.previous_value(key_begin) || value == &self[key_end] {
            return false;
        }

        let overlapped = self.keys_in_range(&key_begin, &key_end);
        let mut end_set = false;
        if !overlapped.is_empty() {
            if !self.value_map.contains_key(key_end) {
                let last_overlap = overlapped.last().unwrap(); // only one element can continue after
                if self.key_exceeds(last_overlap, &key_end) {
                    self.value_map.insert(key_end.clone(), self[last_overlap].clone());
                    end_set = true;
                }
            } 
            overlapped.iter().for_each(|elem| { self.value_map.remove_entry(elem); });
        }
        self.value_map.insert(key_begin.clone(), value.clone());
        if !end_set && !self.value_map.contains_key(key_end) {
            self.value_map.insert(key_end.clone(), self.default_value.clone());
        }
        true
    }

    /// Get the maximum value key (i.e. the last key). Returns None
    /// if the interval map is empty.
    pub fn max_key(&self) -> Option<&K> {
        self.value_map.keys().max()
    }

    /// Get the maximum value key (i.e. the last key). Returns None
    /// if the interval map is empty.
    pub fn min_key(&self) -> Option<&K> {
        self.value_map.keys().min()
    }

    /// If the key domain exceeds a threshold (i.e. the next element is past
    /// the threshold), return true. Otherwise return false.
    /// 
    /// If there is no subsequent key, the key domain obviously passes
    /// the threshold and therefore returns true.
    fn key_exceeds(&self, key: &K, threshold: &K) -> bool {
        match self.next_key(&key) {
            Some(k) => { k > threshold },
            None => { true },
        }
    }

    /// Get all the keys in range in IntervalMap
    fn keys_in_range(&self, start: &K, end: &K) -> Vec<K> {
        self.value_map.keys()
            .filter(|&key| key >= start && key < end)
            .cloned().collect::<Vec<K>>()
    }

    /// For any given key, return the next key. Returns None
    /// if there is no next key.
    fn next_key(&self, key: &K) -> Option<&K> {
        self.value_map.keys().filter(|&k| k > key).min()
    }

    /// For any given key, get the previous key in IntervalMap. Returns None 
    /// if there is no previous key.
    fn previous_key(&self, key: &K) -> Option<&K> {
        self.value_map.keys().filter(|&k| k < key).max()
    }

    /// For any given key, return the next key in IntervalMap's value.
    fn next_value(&self, key: &K) -> &V {
        match self.next_key(key) {
            Some(k) => { &self.value_map.get(&k).unwrap() },
            None => { &self.default_value },
        }
    }

    /// For any given key, return the previous key in IntervalMap's value.
    fn previous_value(&self, key: &K) -> &V {
        match self.previous_key(key) {
            Some(k) => { self.value_map.get(&k).unwrap() },
            None => { &self.default_value },
        }
    }
}

impl<K, V> Index<&K> for IntervalMap<K, V> 
where
    K: Ord + Eq + Hash + Clone,
    V: Eq + Clone,
{
    type Output = V;

    fn index(&self, key: &K) -> &Self::Output {
        let min_key = self.min_key();
        if min_key.is_none() || Some(key) < min_key { // get default
            return &self.default_value;
        } 
        match self.value_map.get(key) {
            Some(val) => { val },
            None => { self.previous_value(key) },
        }
    }
}

impl<K, V> Debug for IntervalMap<K, V> 
where
    K: Ord + Eq + Hash + Debug + Clone,
    V: Eq + Debug + Clone,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map().entries(self.value_map.iter().map(|(k, v)| (k, v))).finish()
    }
}