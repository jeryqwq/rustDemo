use std::{collections::HashMap};

use std::any::Any;

struct Cache {
    cache: HashMap<String, Box<dyn Any>>,
}

impl Cache {
    fn new() -> Self {
        Cache {
            cache: HashMap::new(),
        }
    }
    
    fn insert<T: 'static>(&mut self, key: String, value: T) {
        self.cache.insert(key, Box::new(value));
    }
    
    fn get<T: 'static>(&self, key: &str) -> Option<&T> {
        match self.cache.get(key) {
            Some(value) => value.downcast_ref::<T>(),
            None => None,
        }
    }
    
    fn remove(&mut self, key: &str) {
        self.cache.remove(key);
    }
    
    fn clear(&mut self) {
        self.cache.clear();
    }
    
    fn size(&self) -> usize {
        self.cache.len()
    }
}

fn main() {
    let mut cache = Cache::new();
    
    cache.insert("key1".to_string(), "value1".to_string());
    cache.insert("key2".to_string(), 100);
    cache.insert("key3".to_string(), true);
    
    if let Some(value) = cache.get::<String>("key1") {
        println!("String value: {}", value);
    }
    
    if let Some(value) = cache.get::<i32>("key2") {
        println!("i32 value: {}", value);
    }
    
    if let Some(value) = cache.get::<bool>("key3") {
        println!("bool value: {}", value);
    }
    
    cache.remove("key2");
    
    println!("Cache size: {}", cache.size());
    
    cache.clear();
}


