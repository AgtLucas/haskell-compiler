extern crate collections;
use std::collections::HashMap;
use std::collections::hash_map::{IterMut, RandomState};
use std::hash::Hash;

///A map struct which allows for the introduction of different scopes
///Introducing a new scope will make it possible to introduce additional
///variables with names already defined, shadowing the old name
///After exiting a scope the shadowed variable will again be re introduced
pub struct ScopedMap<K, V> {
    ///A hashmap storing a key -> value mapping
    ///Stores a vector of values in which the value at the top is value returned from 'find'
    map: HashMap<K, Vec<V>>,
    ///A vector of scopes, when entering a scope, None is added as a marker
    ///when later exiting a scope, values are removed from the map until the marker is found
    scopes: Vec<Option<K>>
}

#[allow(dead_code)]
impl <K: Eq + Hash<RandomState> + Clone, V> ScopedMap<K, V> {
    pub fn new() -> ScopedMap<K, V> {
        ScopedMap { map: HashMap::new(), scopes: Vec::new() }
    }
    ///Introduces a new scope
    pub fn enter_scope(&mut self) {
        self.scopes.push(None);
    }
    ///Exits the current scope, removing anything inserted since the
    ///matching enter_scope call
    pub fn exit_scope(&mut self) {
        loop {
            match self.scopes.pop() {
                Some(Some(key)) => { self.map.find_mut(&key).map(|x| x.pop()); }
                _ => break
            }
        }
    }
    ///Removes a previusly inserted value from the map.
    pub fn remove(&mut self, k: &K) -> bool {
        match self.map.find_mut(k).map(|x| x.pop()) {
            Some(..) => {
                let mut i = self.scopes.len() as int - 1;
                while i >= 0 {
                    if self.scopes.get(i as uint).as_ref().map_or(false, |x| x == k) {
                        self.scopes.remove(i as uint);
                    }
                    i -= 1;
                }
                true
            }
            None => false
        }
    }

    ///Returns true if the key has a value declared in the last declared scope
    pub fn in_current_scope(&self, k: &K) -> bool {
        for n in self.scopes.iter().rev() {
            match *n {
                Some(ref name) if name == k => return true,
                None => break,
                _ => ()
            }
        }
        false
    }
    ///Returns an iterator of the (key, values) pairs inserted in the map
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, K, Vec<V>> {
        self.map.iter_mut()
    }

    ///Returns a reference to the last inserted value corresponding to the key
    fn find<'a>(&'a self, k: &K) -> Option<&'a V> {
        self.map.find(k).and_then(|x| x.last())
    }

    ///Returns the number of elements in the container.
    ///Shadowed elements are not counted
    fn len(&self) -> uint { self.map.len() }

    ///Removes all elements
    fn clear(&mut self) {
        self.map.clear();
        self.scopes.clear();
    }

    ///Swaps the value stored at key, or inserts it if it is not present
    fn swap(&mut self, k: K, v: V) -> Option<V> {
        let vec = self.map.find_or_insert(k.clone(), Vec::new());
        if vec.len() != 0 {
            let r  = vec.pop();
            vec.push(v);
            r
        }
        else {
            vec.push(v);
            self.scopes.push(Some(k));
            None
        }
    }
    fn pop(&mut self, k: &K) -> Option<V> {
        match self.map.find_mut(k).and_then(|x| x.pop()) {
            Some(v) => {
                let mut i = self.scopes.len() as int - 1;
                while i >= 0 {
                    if self.scopes.get(i as uint).as_ref().map_or(false, |x| x == k) {
                        self.scopes.remove(i as uint);
                    }
                    i -= 1;
                }
                Some(v)
            }
            None => None
        }
    }
    fn find_mut<'a>(&'a mut self, key: &K) -> Option<&'a mut V> {
        self.map.find_mut(key).and_then(|x| x.mut_last())
    }
    fn insert(&mut self, k: K, v: V) -> bool {
        let vec = self.map.find_or_insert(k.clone(), Vec::new());
        vec.push(v);
        self.scopes.push(Some(k));
        vec.len() == 1
    }
}

#[cfg(test)]
mod tests {
    use scoped_map::ScopedMap;
    #[test]
    fn test() {
        let mut map = ScopedMap::new();
        map.insert("a", 0);
        map.insert("b", 1);
        map.enter_scope();
        assert_eq!(map.find(&"a"), Some(&0));
        assert_eq!(map.find(&"b"), Some(&1));
        assert_eq!(map.find(&"c"), None);
        map.insert("a", 1);
        map.insert("c", 2);
        assert_eq!(map.find(&"a"), Some(&1));
        assert_eq!(map.find(&"c"), Some(&2));
        map.exit_scope();
        assert_eq!(map.find(&"a"), Some(&0));
        assert_eq!(map.find(&"c"), None);
    }
}
