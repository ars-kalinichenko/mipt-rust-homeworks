#![feature(generic_associated_types)]
use ntest::timeout;
use rand::{distributions::Alphanumeric, Rng};
use std::collections::{hash_map::Entry::*, HashMap};
use trie::{trie::Trie, trie_key::ToKeyIter};

struct Number(u32);

struct NumberIter(u32, u8);

impl Iterator for NumberIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.1 == 32 || (self.0 >> self.1) == 0 {
            None
        } else {
            let ret = ((self.0 >> self.1) as u8) & 0xFF;
            self.1 += 8;
            Some(ret)
        }
    }
}

impl ToKeyIter for Number {
    type Item = u8;
    type KeyIter<'a> = NumberIter;

    fn key_iter<'a>(&'a self) -> Self::KeyIter<'a> {
        NumberIter(self.0, 0)
    }
}

////////////////////////////////////////////////////////////////////////////////

struct TrieLike {
    data: HashMap<String, (usize, Option<i32>)>,
}

impl TrieLike {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }

    fn len(&self) -> usize {
        self.data.get("").map(|x| x.0).unwrap_or_default()
    }

    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    fn insert(&mut self, key: String, value: i32) -> Option<i32> {
        let old_value = match self.data.entry(key.clone()) {
            Occupied(mut entry) => entry.get_mut().1.replace(value),
            Vacant(entry) => {
                entry.insert((0, Some(value)));
                None
            }
        };
        if old_value.is_some() {
            return old_value;
        }
        for count in 0..(key.len() + 1) {
            match self.data.entry(key.chars().take(count).collect()) {
                Occupied(mut entry) => {
                    entry.get_mut().0 += 1;
                }
                Vacant(entry) => {
                    entry.insert((1, None));
                }
            }
        }
        old_value
    }

    fn remove(&mut self, key: &str) -> Option<i32> {
        match self.data.entry(key.to_string()) {
            Occupied(mut entry) if entry.get().1.is_some() => {
                let old_value = entry.get_mut().1.take();
                for count in 0..(key.len() + 1) {
                    match self.data.entry(key.chars().take(count).collect()) {
                        Occupied(entry) if entry.get().0 == 1 => {
                            entry.remove();
                        }
                        Occupied(mut entry) => {
                            entry.get_mut().0 -= 1;
                        }
                        _ => {}
                    }
                }
                old_value
            }
            _ => None,
        }
    }

    fn contains(&self, key: &str) -> bool {
        self.data.get(key).map_or(false, |t| t.1.is_some())
    }

    fn get(&self, key: &str) -> Option<&i32> {
        self.data.get(key).map(|x| x.1.as_ref()).flatten()
    }

    fn get_mut(&mut self, key: &str) -> Option<&mut i32> {
        self.data.get_mut(key).map(|x| x.1.as_mut()).flatten()
    }

    fn starts_with(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
}

////////////////////////////////////////////////////////////////////////////////

fn compare_to_map(max_str_len: usize, iter_count: usize) {
    let mut trie = Trie::<String, i32>::new();
    let mut naive = TrieLike::new();
    let mut rng = rand::thread_rng();
    for _ in 0..iter_count {
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rng.gen_range(1usize..max_str_len))
            .map(char::from)
            .collect();
        let value = rng.gen::<i32>();
        match rng.gen_range(0usize..10) {
            0..=7 => {
                assert_eq!(trie.insert(&key, value), naive.insert(key.clone(), value));
            }
            8..=9 => {
                assert_eq!(trie.remove(&key), naive.remove(&key));
            }
            _ => unreachable!(),
        }
        assert_eq!(trie.len(), naive.len());
        assert_eq!(trie.is_empty(), naive.is_empty());
        assert_eq!(trie.starts_with(&key), naive.starts_with(&key));
        assert_eq!(trie.contains(&key), naive.contains(&key));
        assert_eq!(trie.get(&key), naive.get(&key));
        assert_eq!(trie.get_mut(&key), naive.get_mut(&key));
    }
}

////////////////////////////////////////////////////////////////////////////////

#[test]
fn empty() {
    let trie = Trie::<String, i32>::new();
    assert_eq!(trie.get("a"), None);
}

#[test]
fn should_compile() {
    let mut trie = Trie::<String, i32>::new();
    let key = "hello world".to_owned();
    assert_eq!(trie.insert("hello world", 42), None);
    assert!(trie.contains("hello world"));
    assert!(trie.contains(&key));
    assert!(!trie.contains(&"hello world"[0..5]));
    assert_eq!(trie.get("hello world"), Some(&42));
    assert_eq!(trie["hello world"], 42);
    assert_eq!(trie.get("hello "), None);
}

#[test]
fn it_works1() {
    let mut trie = Trie::<String, i32>::new();
    assert_eq!(trie.insert("apple", 1), None);
    assert_eq!(trie.get("apple"), Some(&1));
    assert_eq!(trie.get("app"), None);
    assert!(trie.starts_with("app"));
    assert_eq!(trie.insert("app", 2), None);
    assert_eq!(trie.get("app"), Some(&2));
    *trie.get_mut("app").unwrap() = 42;
    assert_eq!(trie.get("app"), Some(&42));
}

#[test]
fn it_works2() {
    let mut trie = Trie::<String, i32>::new();
    assert_eq!(trie.insert("abcd", 1), None);
    assert_eq!(trie.get("abcd"), Some(&1));
    assert!(trie.starts_with("abcd"));
    assert_eq!(trie.get("ab"), None);

    assert_eq!(trie.insert("ab", 2), None);
    assert_eq!(trie.get("abcd"), Some(&1));
    assert!(trie.starts_with("abcd"));
    assert_eq!(trie.get("ab"), Some(&2));

    assert_eq!(trie.remove("abcd"), Some(1));
    assert_eq!(trie.get("abcd"), None);
    assert_eq!(trie.get("abc"), None);
    assert_eq!(trie.get("ab"), Some(&2));
    assert_eq!(trie.get("a"), None);
    assert_eq!(trie.get(""), None);

    assert!(!trie.starts_with("abcde"));
    assert!(!trie.starts_with("abcd"));
    assert!(!trie.starts_with("abc"));
    assert!(trie.starts_with("ab"));
    assert!(trie.starts_with("a"));
    assert!(trie.starts_with(""));
}

#[test]
fn it_works3() {
    let mut trie = Trie::<Number, i32>::new();
    trie.insert(&Number(33022), 1); // 254 128
    trie.insert(&Number(855163), 2); // 123 12 13
    trie.insert(&Number(795390), 4); // 254 34 12
    assert!(trie.starts_with(&Number(254))); // 254
    assert!(trie.contains(&Number(33022))); // 254 128
    assert!(trie.contains(&Number(855163))); // 123 12 13
    assert!(trie.contains(&Number(795390))); // 254 34 12
    assert!(trie.starts_with(&Number(254))); // 254
    assert!(!trie.contains(&Number(254))); // 254
    assert!(trie.starts_with(&Number(123))); // 123
    assert!(!trie.contains(&Number(123))); // 123
    assert!(!trie.starts_with(&Number(3326))); // 254 12
    assert!(!trie.contains(&Number(3326))); // 254 12
}

#[test]
fn it_works4() {
    let mut trie = Trie::<String, i32>::new();
    trie.insert("saF", 1);
    assert_eq!(trie.len(), 1);
    trie.insert("aFS", 2);
    assert_eq!(trie.len(), 2);
    trie.insert("aFS", 4);
    assert_eq!(trie.len(), 2);
    assert_eq!(trie.get("aFS"), Some(&4));
    trie.insert("aF", 3);
    assert_eq!(trie.len(), 3);
    trie.remove("aFS");
    assert!(!trie.starts_with("aFS"));
    assert!(trie.starts_with("aF"));
    trie.remove("aF");
    assert!(!trie.starts_with("aF"));
}

#[test]
fn it_works5() {
    let mut trie = Trie::<String, i32>::new();
    assert_eq!(trie.len(), 0);
    trie.insert(&"ab".to_owned(), 5);
    assert_eq!(trie.len(), 1);
    trie.insert(&"ab".to_owned(), 4);
    assert_eq!(trie.len(), 1);
}

#[test]
fn stress1() {
    for _ in 0..1000 {
        compare_to_map(6, 10000)
    }
}

#[test]
fn stress2() {
    for _ in 0..25 {
        compare_to_map(100, 10000)
    }
}

#[test]
#[timeout(7500)]
fn performance() {
    let mut trie = Trie::<String, i32>::new();
    let mut rng = rand::thread_rng();
    for _ in 0..6000 {
        let key: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(rng.gen_range(0usize..5000))
            .map(char::from)
            .collect();
        let value = rng.gen::<i32>();
        match rng.gen_range(0usize..10) {
            0..=7 => {
                trie.insert(&key, value);
            }
            8..=9 => {
                trie.starts_with(&key);
            }
            _ => unreachable!(),
        }
    }
}
