use bst::AVLTreeMap;
use ntest::timeout;
use rand::{seq::SliceRandom as _, Rng as _};
use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Number(i32);

#[test]
fn empty() {
    let mut map = AVLTreeMap::new();
    assert!(map.is_empty());
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(2, 2), None);
    assert_eq!(map.insert(3, 3), None);
    assert!(!map.is_empty());
}

#[test]
fn should_compile1() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.insert(Number(1), 1), None);
    assert!(map.contains_key(&Number(1)));
}

#[test]
fn should_compile2() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.remove("hello"), None);
    assert_eq!(map.insert("hello".to_string(), 1), None);
    assert!(map.contains_key("hello"));
    assert!(!map.contains_key("world"));
    assert_eq!(map.remove_entry("hello"), Some(("hello".to_string(), 1)));
}

#[test]
fn contains() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(2, 2), None);
    assert_eq!(map.insert(3, 3), None);
    assert!(!map.contains_key(&0));
    assert!(map.contains_key(&1));
    assert!(map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert!(!map.contains_key(&4));
}

#[test]
fn remove() {
    let mut map = AVLTreeMap::new();
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(2, 2), None);
    assert_eq!(map.insert(3, 3), None);
    assert_eq!(map.remove(&1), Some(1));
    assert!(!map.contains_key(&1));
    assert!(map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert_eq!(map.remove(&2), Some(2));
    assert!(!map.contains_key(&1));
    assert!(!map.contains_key(&2));
    assert!(map.contains_key(&3));
    assert_eq!(map.remove(&3), Some(3));
    assert!(!map.contains_key(&1));
    assert!(!map.contains_key(&2));
    assert!(!map.contains_key(&3));
    assert!(map.is_empty());
}

#[test]
fn test_nth() {
    let mut map = AVLTreeMap::<u8, u8>::new();
    assert_eq!(map.insert(2, 2), None);
    assert_eq!(map.insert(1, 1), None);
    assert_eq!(map.insert(3, 3), None);

    assert_eq!(map.remove_entry(&2), Some((2, 2)));
    assert_eq!(map.insert(2, 2), None);

    assert_eq!(map.nth_key_value(0), Some((&1, &1)));
    assert_eq!(map.nth_key_value(1), Some((&2, &2)));
    assert_eq!(map.nth_key_value(2), Some((&3, &3)));
}

#[test]
#[timeout(1500)]
fn performance1() {
    let count = 10000000;
    let mut rng = rand::thread_rng();
    let mut map = AVLTreeMap::new();
    let mut hash_map = HashMap::<u8, u8>::new();
    for _ in 0..count {
        let key = rng.gen();
        let value = rng.gen();
        map.insert(key, value);
        hash_map.insert(key, value);
    }
    let mut vec: Vec<_> = hash_map.into_iter().collect();
    vec.sort_unstable();
    let mut vec: Vec<_> = vec
        .into_iter()
        .enumerate()
        .map(|(index, (key, value))| (key, value, index))
        .collect();
    vec.shuffle(&mut rng);
    for (key, value, index) in &vec {
        assert!(map.contains_key(key));
        assert_eq!(map.nth_key_value(*index), Some((key, value)));
        assert_eq!(map.get_key_value(key), Some((key, value)));
    }
    for (key, value, _) in &vec {
        assert_eq!(map.remove_entry(key), Some((*key, *value)));
        assert!(!map.contains_key(key));
    }
}

#[test]
#[timeout(2500)]
fn performance2() {
    let count = 8000000;
    let mut rng = rand::thread_rng();
    let mut map = AVLTreeMap::new();
    let mut hash_map = HashMap::<u8, u8>::new();
    for _ in 0..count {
        let key = rng.gen();
        let value = rng.gen();
        match rng.gen_range(0usize..10) {
            0..=7 => {
                assert_eq!(map.insert(key, value), hash_map.insert(key, value));
            }
            8 => {
                assert_eq!(map.remove(&key), hash_map.remove(&key));
            }
            9 => {
                assert_eq!(map.remove_entry(&key), hash_map.remove_entry(&key));
            }
            _ => unreachable!(),
        }
        assert_eq!(map.is_empty(), hash_map.is_empty());
        assert_eq!(map.len(), hash_map.len());
        assert_eq!(map.contains_key(&key), hash_map.contains_key(&key));
        assert_eq!(map.get(&key), hash_map.get(&key));
        assert_eq!(map.get_key_value(&key), hash_map.get_key_value(&key));
    }
}

#[test]
#[timeout(1500)]
fn performance3() {
    let count = 1000000;
    let mut rng = rand::thread_rng();
    let mut map = AVLTreeMap::<i32, i32>::new();
    for i in 0..count {
        let value = rng.gen();
        map.insert(i, value);
    }
    for _ in 0..count {
        assert_eq!(map.contains_key(&count), false);
    }
    for i in 1000..count {
        map.remove(&i);
    }
    for i in 0..count {
        let value = rng.gen();
        map.insert(-i, value);
    }
    for _ in 0..count {
        assert_eq!(map.contains_key(&(-count)), false);
    }
}
