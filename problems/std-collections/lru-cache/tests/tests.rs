use lru_cache::LRUCache;
use ntest::timeout;
use rand::Rng;

struct NaiveLRUCache<K, V> {
    capacity: usize,
    cache: Vec<(K, V)>,
}

impl<K: Eq, V> NaiveLRUCache<K, V> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0);
        Self {
            capacity,
            cache: Vec::new(),
        }
    }

    pub fn get(&mut self, key: &K) -> Option<&V> {
        let index = self.cache.iter().position(|(a, _)| a == key)?;
        let pair = self.cache.remove(index);
        self.cache.push(pair);
        Some(&self.cache.last().unwrap().1)
    }

    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(index) = self.cache.iter().position(|(a, _)| a == &key) {
            let (_, old_value) = self.cache.remove(index);
            self.cache.push((key, value));
            Some(old_value)
        } else {
            if self.cache.len() == self.capacity {
                self.cache.remove(0);
            }
            self.cache.push((key, value));
            None
        }
    }
}

#[test]
#[should_panic]
fn check_zero_capacity() {
    LRUCache::<i32, i32>::new(0);
}

#[test]
fn should_compile() {
    #[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
    struct Key {
        _key: i32,
    }
    struct Value {
        _value: i32,
    }
    LRUCache::<Key, Value>::new(1);
}

#[test]
fn it_works1() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.insert(1, 1), None);
    assert_eq!(cache.insert(2, 2), None);
    assert_eq!(cache.get(&1), Some(&1));
    assert_eq!(cache.insert(3, 3), None);
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.insert(4, 4), None);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&3), Some(&3));
    assert_eq!(cache.get(&4), Some(&4));
}

#[test]
fn it_works2() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.get(&2), None);
    assert_eq!(cache.insert(2, 6), None);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.insert(1, 5), None);
    assert_eq!(cache.insert(1, 2), Some(5));
    assert_eq!(cache.get(&1), Some(&2));
    assert_eq!(cache.get(&2), Some(&6));
}

#[test]
fn it_works3() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.insert(2, 1), None);
    assert_eq!(cache.insert(2, 2), Some(1));
    assert_eq!(cache.get(&2), Some(&2));
    assert_eq!(cache.insert(1, 1), None);
    assert_eq!(cache.insert(4, 1), None);
    assert_eq!(cache.get(&2), None);
}

#[test]
fn it_works4() {
    let mut cache = LRUCache::new(2);
    assert_eq!(cache.insert(2, 1), None);
    assert_eq!(cache.insert(1, 1), None);
    assert_eq!(cache.get(&2), Some(&1));
    assert_eq!(cache.insert(4, 1), None);
    assert_eq!(cache.get(&1), None);
    assert_eq!(cache.get(&2), Some(&1));
}

#[test]
fn small_capacity() {
    let mut cache = LRUCache::new(10);
    let mut naive = NaiveLRUCache::new(10);
    let mut rng = rand::thread_rng();
    for _ in 0..500000 {
        if rng.gen_range(0..3) == 0 {
            let key = rng.gen_range(0..30);
            assert_eq!(cache.get(&key), naive.get(&key));
        } else {
            let key = rng.gen_range(0..30);
            let value = rng.gen::<i8>();
            assert_eq!(cache.insert(key, value), naive.insert(key, value));
        }
    }
}

#[test]
fn big_capacity() {
    let mut cache = LRUCache::new(1000);
    let mut naive = NaiveLRUCache::new(1000);
    let mut rng = rand::thread_rng();
    for _ in 0..500000 {
        if rng.gen_range(0..3) == 0 {
            let key = rng.gen_range(0..3000);
            assert_eq!(cache.get(&key), naive.get(&key));
        } else {
            let key = rng.gen_range(0..3000);
            let value = rng.gen::<i8>();
            assert_eq!(cache.insert(key, value), naive.insert(key, value));
        }
    }
}

#[test]
#[timeout(4000)]
fn stress() {
    let mut cache = LRUCache::new(100000);
    let mut rng = rand::thread_rng();
    for _ in 0..500000 {
        if rng.gen_range(0..3) == 0 {
            let key = rng.gen_range(0..1000000);
            cache.get(&key);
        } else {
            let key = rng.gen_range(0..1000000);
            let value = rng.gen::<i8>();
            cache.insert(key, value);
        }
    }
}
