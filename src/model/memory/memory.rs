use std::any::Any;
use std::collections::HashMap;

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Owner {
    TABLE,
    PLAYERCOLLECTION(Vec<String>),
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct MemoryKey {
    name: String,
    owner: Option<Owner>,
}

/// Manages Memory for Mental Card Games
#[derive(Debug)]
pub struct Memory {
    value_map:HashMap<MemoryKey, Box<dyn Any>>,
}

impl Memory {
    
    /// Creates new Memory
    pub fn new() -> Memory {
        Self {
            value_map: HashMap::new(),
        }
    }
    
    /// Inserts value into map
    /// 
    /// If for the `key` there already was a value present return value
    /// 
    /// Else returns None
    pub fn insert<V: Any>(&mut self, key: String, value: V, owner: Option<Owner>) -> Option<V> {
        let mem_key = MemoryKey { name: key.to_string(), owner};
        
        let boxed = self.value_map
            .insert(mem_key, Box::new(value))?
            .downcast::<V>()
            .ok()?;
        
        Some(*boxed)
    }
    
    /// Gets a reference to a value for a given key
    /// 
    /// If `key` is not present return None
    /// 
    /// If `V` given doesn't match value return None
    pub fn get<V: Any>(&self, key: String, owner: Option<Owner>) -> Option<&V> {
        let mem_key = MemoryKey { name: key.to_string(), owner};
        self.value_map.get(&mem_key)?
            .downcast_ref::<V>()
    }
    
    /// Gets a mutable reference to a value for a given key.
    /// 
    /// If `key` is not present return None
    /// 
    /// If `V` doesn't match value return None
    pub fn get_mut<V: Any>(&mut self, key: String, owner: Option<Owner>) -> Option<&mut V> {
        let mem_key = MemoryKey { name: key.to_string(), owner};
        self.value_map.get_mut(&mem_key)?
            .downcast_mut::<V>()
    }
    
    pub fn get_by_key<V: Any>(&mut self, key:String) -> Vec<(&V, Option<&Owner>)> {
        self.value_map.keys()
            .filter(|&mem_key| mem_key.name == key)
            .map(|mem_key| {
                let value = self.value_map.get(mem_key).unwrap().downcast_ref::<V>().unwrap();
                (value, mem_key.owner.as_ref())
            })
            .collect()
    }
    
    /// Removes value from map for given key
    pub fn remove<V: Any>(&mut self, key: String, owner: Option<Owner>) -> Option<V> {
        let mem_key = MemoryKey { name: key.to_string(), owner};
        let boxed = self.value_map.remove(&mem_key)?
            .downcast::<V>()
            .ok()?;
        Some(*boxed)
    }
    
    pub fn clear(&mut self) {
        self.value_map.clear();
    }
    
    pub fn is_empty(&self) -> bool {
        self.value_map.is_empty()
    }
    
    pub fn len(&self) -> usize {
        self.value_map.len()
    }
}

impl Clone for Memory {
    fn clone(&self) -> Memory {
        // Can't clone Memory creates new Memory
        Memory::new()
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
    }
    
    #[test]
    fn test_insert_memory() {
        let mut memory: Memory = Memory::new();

        assert_eq!(memory.insert("key".to_string(), "value".to_string(), None), None);
        assert_eq!(memory.get("key".to_string(), None), Some(&"value".to_string()));
    }
    
    #[test]
    fn test_get_memory() {
        let mut memory: Memory= Memory::new();
        // Insert String
        memory.insert("key0".to_string(), "value".to_string(), None);
        memory.insert("key0".to_string(), "value".to_string(), Some(Owner::TABLE));
        memory.insert("key0".to_string(), "value".to_string(), Some(Owner::PLAYERCOLLECTION(vec![String::from("player1")])));
        memory.insert("key1".to_string(), "value1", None);
        // Insert Int
        memory.insert("key2".to_string(), 1u8, None);
        memory.insert("key3".to_string(), 21isize, None);
        
        assert_eq!(memory.get::<i8>("none".to_string(), None), None);
        assert_eq!(memory.get::<String>("key0".to_string(), None), Some(&"value".to_string()));
        assert_eq!(memory.get::<String>("key0".to_string(), Some(Owner::TABLE)), Some(&"value".to_string()));
        assert_eq!(memory.get::<String>("key0".to_string(), Some(Owner::PLAYERCOLLECTION(vec![String::from("player1")]))), Some(&"value".to_string()));
        assert_eq!(memory.get::<&str>("key1".to_string(), None), Some(&"value1"));
        assert_eq!(memory.get::<u8>("key2".to_string(), None), Some(&1u8));
        assert_eq!(memory.get::<isize>("key3".to_string(), None), Some(&21isize));
    }

    #[test]
    fn test_get_mut_memory() {
        let mut memory: Memory= Memory::new();
        // Insert String
        memory.insert("key0".to_string(), "value".to_string(), None);
        memory.insert("key1".to_string(), "value1", None);
        // Insert Int
        memory.insert("key2".to_string(), 1u8, None);
        memory.insert("key3".to_string(), 21isize, None);

        assert_eq!(memory.get_mut::<i8>("none".to_string(), None), None);
        assert_eq!(memory.get_mut::<String>("key0".to_string(), None), Some(&mut "value".to_string()));
        assert_eq!(memory.get_mut::<&str>("key1".to_string(), None), Some(&mut "value1"));
        assert_eq!(memory.get_mut::<u8>("key2".to_string(), None), Some(&mut 1u8));
        assert_eq!(memory.get_mut::<isize>("key3".to_string(), None), Some(&mut 21isize));
    }

    #[test]
    fn test_remove_memory() {
        let mut memory: Memory = Memory::new();
        // Insert String
        memory.insert("key0".to_string(), "value".to_string(), None);
        memory.insert("key1".to_string(), "value1", None);
        // Insert Int
        memory.insert("key2".to_string(), 1u8, None);
        memory.insert("key3".to_string(), 21isize, None);

        assert_eq!(memory.remove::<i8>("none".to_string(), None), None);
        assert_eq!(memory.len(), 4);
        assert_eq!(memory.remove::<String>("key0".to_string(), None), Some("value".to_string()));
        assert_eq!(memory.len(), 3);
        assert_eq!(memory.remove::<&str>("key1".to_string(), None), Some("value1"));
        assert_eq!(memory.len(), 2);
        assert_eq!(memory.remove::<u8>("key2".to_string(), None), Some(1u8));
        assert_eq!(memory.len(), 1);
        assert_eq!(memory.remove::<isize>("key3".to_string(), None), Some(21isize));
        assert!(memory.is_empty());
    }
}