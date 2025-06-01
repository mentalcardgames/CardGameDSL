use std::any::Any;
use std::collections::HashMap;

/// Manages Memory for Mental Card Games
pub struct Memory {
    value_map:HashMap<String, Box<dyn Any>>,
}

impl Memory {
    
    /// Creates new Memory
    pub fn new() -> Memory {
        Self {
            value_map: HashMap::new(),
        }
    }
    
    /// Creates new Memory with initial capacity
    pub fn new_with_capacity(capacity: usize) -> Memory {
        Self {
            value_map: HashMap::with_capacity(capacity),
        }
    }
    
    
    /// Inserts value into map
    /// 
    /// If for the `key` there already was a value present return value
    /// 
    /// Else returns None
    pub fn insert<V: Any>(&mut self, key: String, value: V) -> Option<V> {
        let boxed = self.value_map
            .insert(key.clone(), Box::new(value))?
            .downcast::<V>()
            .ok()?;
        
        Some(*boxed)
    }
    
    /// Gets a reference to a value for a given key
    /// 
    /// If `key` is not present return None
    /// 
    /// If `V` given doesn't match value return None
    pub fn get<V: Any>(&self, key: String) -> Option<&V> {
        self.value_map.get(&key.clone())?
            .downcast_ref::<V>()
    }
    
    /// Gets a mutable reference to a value for a given key.
    /// 
    /// If `key` is not present return None
    /// 
    /// If `V` doesn't match value return None
    pub fn get_mut<V: Any>(&mut self, key: String) -> Option<&mut V> {
        self.value_map.get_mut(&key.clone())?
            .downcast_mut::<V>()
    }
    
    /// Removes value from map for given key
    pub fn remove<V: Any>(&mut self, key: String) -> Option<V> {
        let boxed = self.value_map.remove(&key.clone())?
            .downcast::<V>()
            .ok()?;
        
        Some(*boxed)
    }
    
    pub fn clear(&mut self) {
        self.value_map.clear();
    }
    
    pub fn capacity(&self) -> usize {
        self.value_map.capacity()
    }
    
    pub fn is_empty(&self) -> bool {
        self.value_map.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.value_map.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_create_memory() {
        // Test create without capacity
        let mut memory: Memory = Memory::new();
        assert_eq!(memory.len(), 0);
        assert!(memory.is_empty());
        
        // Test create with capacity
        let mut mem_cap: Memory = Memory::new_with_capacity(10);
        assert_eq!(mem_cap.capacity(), 10);
        assert!(mem_cap.is_empty());
    }
    
    #[test]
    fn test_insert_memory() {
        let mut memory: Memory = Memory::new();

        assert_eq!(memory.insert("key".to_string(), "value".to_string()), None);
        assert_eq!(memory.get("key".to_string()), Some(&"value".to_string()));
    }
    
    #[test]
    fn test_get_memory() {
        let mut memory: Memory= Memory::new();
        // Insert String
        memory.insert("key0".to_string(), "value".to_string());
        memory.insert("key1".to_string(), "value1");
        // Insert Int
        memory.insert("key2".to_string(), 1u8);
        memory.insert("key3".to_string(), 21isize);
        
        assert_eq!(memory.get::<i8>("none".to_string()), None);
        assert_eq!(memory.get::<String>("key0".to_string()), Some(&"value".to_string()));
        assert_eq!(memory.get::<&str>("key1".to_string()), Some(&"value1"));
        assert_eq!(memory.get::<u8>("key2".to_string()), Some(&1u8));
        assert_eq!(memory.get::<isize>("key3".to_string()), Some(&21isize));
    }

    #[test]
    fn test_get_mut_memory() {
        let mut memory: Memory= Memory::new();
        // Insert String
        memory.insert("key0".to_string(), "value".to_string());
        memory.insert("key1".to_string(), "value1");
        // Insert Int
        memory.insert("key2".to_string(), 1u8);
        memory.insert("key3".to_string(), 21isize);

        assert_eq!(memory.get_mut::<i8>("none".to_string()), None);
        assert_eq!(memory.get_mut::<String>("key0".to_string()), Some(&mut "value".to_string()));
        assert_eq!(memory.get_mut::<&str>("key1".to_string()), Some(&mut "value1"));
        assert_eq!(memory.get_mut::<u8>("key2".to_string()), Some(&mut 1u8));
        assert_eq!(memory.get_mut::<isize>("key3".to_string()), Some(&mut 21isize));
    }

    #[test]
    fn test_remove_memory() {
        let mut memory: Memory = Memory::new();
        // Insert String
        memory.insert("key0".to_string(), "value".to_string());
        memory.insert("key1".to_string(), "value1");
        // Insert Int
        memory.insert("key2".to_string(), 1u8);
        memory.insert("key3".to_string(), 21isize);

        assert_eq!(memory.remove::<i8>("none".to_string()), None);
        assert_eq!(memory.len(), 4);
        assert_eq!(memory.remove::<String>("key0".to_string()), Some("value".to_string()));
        assert_eq!(memory.len(), 3);
        assert_eq!(memory.remove::<&str>("key1".to_string()), Some("value1"));
        assert_eq!(memory.len(), 2);
        assert_eq!(memory.remove::<u8>("key2".to_string()), Some(1u8));
        assert_eq!(memory.len(), 1);
        assert_eq!(memory.remove::<isize>("key3".to_string()), Some(21isize));
        assert!(memory.is_empty());
    }
}