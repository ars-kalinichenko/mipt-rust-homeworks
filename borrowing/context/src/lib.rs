#![forbid(unsafe_code)]

use std::any::{Any, TypeId};
use std::collections::HashMap;

pub struct Context {
    /*
    Мы используем два хранилища. Один для синглтонов, другой для key-object
     */
    key_value_storage: HashMap<String, Box<dyn Any>>,
    singleton_storage: HashMap<TypeId, Box<dyn Any>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            key_value_storage: HashMap::new(),
            singleton_storage: HashMap::new(),
        }
    }

    pub fn insert<T: Any + 'static>(&mut self, key: &str, obj: T) {
        self.key_value_storage.insert(key.to_string(), Box::new(obj));
    }

    pub fn get<T: Any + 'static>(&self, key: &str) -> &T {
        match self.key_value_storage.get(key)
            .and_then(|boxed| boxed.downcast_ref::<T>())
        {
            Some(x) => x,
            None => panic!("Panic!"),
        }
    }

    pub fn insert_singleton<T: Any + 'static>(&mut self, obj: T) {
        let object_type = TypeId::of::<T>();
        self.singleton_storage.insert(object_type, Box::new(obj));
    }

    pub fn get_singleton<T: Any + 'static>(&self) -> &T {
        match self
            .singleton_storage
            .get(&TypeId::of::<T>())
            .and_then(|boxed| boxed.downcast_ref::<T>())
        {
            Some(x) => x,
            None => panic!("Panic!"),
        }
    }
}
