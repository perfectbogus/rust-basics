// Advanced Ownership Challenge: Multi-threaded Cache System
// Your task: Make ALL these tests pass!

use std::rc::Rc;
use std::cell::RefCell;

// TODO: Define your SharedCache struct and any supporting types here
struct SharedCache<T> {
    key: String,
    value: Rc<RefCell<T>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_storage_and_retrieval() {
        let mut cache = SharedCache::new();

        // Should be able to store owned data
        cache.insert("key1".to_string(), "value1".to_string());

        // Should be able to retrieve a reference
        let value_ref = cache.get("key1").unwrap();
        assert_eq!(*value_ref, "value1");

        // Original value should still exist after getting reference
        let value_ref2 = cache.get("key1").unwrap();
        assert_eq!(*value_ref2, "value1");
    }

    #[test]
    fn test_multiple_references_to_same_item() {
        let mut cache = SharedCache::new();
        cache.insert("shared".to_string(), vec![1, 2, 3, 4]);

        // Get multiple references to the same item
        let ref1 = cache.get("shared").unwrap();
        let ref2 = cache.get("shared").unwrap();
        let ref3 = cache.get("shared").unwrap();

        // All should point to the same data
        assert_eq!(ref1.len(), 4);
        assert_eq!(ref2.len(), 4);
        assert_eq!(ref3.len(), 4);

        // Should be able to use all references simultaneously
        println!("Ref1: {:?}, Ref2: {:?}, Ref3: {:?}", ref1, ref2, ref3);
    }

    #[test]
    fn test_shared_cache_between_multiple_owners() {
        // Two different variables should be able to share the same cache
        let cache1 = SharedCache::new();
        let cache2 = cache1.clone(); // This should share, not deep copy

        // Modify through cache1
        cache1.insert("shared_key".to_string(), 42);

        // Should see change through cache2
        let value = cache2.get("shared_key").unwrap();
        assert_eq!(*value, 42);

        // Modify through cache2
        cache2.insert("another_key".to_string(), 100);

        // Should see change through cache1
        let value2 = cache1.get("another_key").unwrap();
        assert_eq!(*value2, 100);
    }

    #[test]
    fn test_reference_counting() {
        let mut cache = SharedCache::new();
        cache.insert("counted".to_string(), "data".to_string());

        // Should start with 0 active references
        assert_eq!(cache.active_references("counted"), 0);

        {
            let _ref1 = cache.get("counted").unwrap();
            assert_eq!(cache.active_references("counted"), 1);

            {
                let _ref2 = cache.get("counted").unwrap();
                let _ref3 = cache.get("counted").unwrap();
                assert_eq!(cache.active_references("counted"), 3);
            }
            // ref2 and ref3 dropped
            assert_eq!(cache.active_references("counted"), 1);
        }
        // All references dropped
        assert_eq!(cache.active_references("counted"), 0);
    }

    #[test]
    fn test_modify_while_references_exist() {
        let mut cache = SharedCache::new();
        cache.insert("key".to_string(), vec![1, 2]);

        // Get a reference
        let reference = cache.get("key").unwrap();
        assert_eq!(reference.len(), 2);

        // Should be able to insert OTHER items while reference exists
        cache.insert("other_key".to_string(), vec![3, 4]);

        // Original reference should still be valid
        assert_eq!(reference.len(), 2);
        assert_eq!(reference[0], 1);

        // Should be able to get reference to new item
        let other_ref = cache.get("other_key").unwrap();
        assert_eq!(other_ref.len(), 2);
        assert_eq!(other_ref[0], 3);
    }

    #[test]
    fn test_borrowing_external_data() {
        let external_data = "I live outside the cache".to_string();
        let mut cache = SharedCache::new();

        // Should be able to store a reference to external data
        // This is the really tricky part - storing borrowed data!
        cache.insert_borrowed("external", &external_data);

        let cached_ref = cache.get("external").unwrap();
        assert_eq!(*cached_ref, "I live outside the cache");

        // The external data should still be accessible
        assert_eq!(external_data, "I live outside the cache");
    }

    #[test]
    fn test_cache_size_and_cleanup() {
        let mut cache = SharedCache::new();

        assert_eq!(cache.size(), 0);

        cache.insert("item1".to_string(), 1);
        cache.insert("item2".to_string(), 2);
        assert_eq!(cache.size(), 2);

        // Should be able to remove items
        cache.remove("item1");
        assert_eq!(cache.size(), 1);

        // Should return None for removed items
        assert!(cache.get("item1").is_none());
        assert!(cache.get("item2").is_some());
    }

    #[test]
    fn test_complex_borrowing_scenario() {
        let cache1 = SharedCache::new();
        let cache2 = cache1.clone();

        // Insert through cache1
        cache1.insert("data".to_string(), vec![1, 2, 3]);

        // Get reference through cache2
        let ref_from_cache2 = cache2.get("data").unwrap();

        // Insert more data through cache1 while reference from cache2 exists
        cache1.insert("more_data".to_string(), vec![4, 5, 6]);

        // Both references should work
        assert_eq!(ref_from_cache2.len(), 3);
        let ref_to_new = cache1.get("more_data").unwrap();
        assert_eq!(ref_to_new.len(), 3);

        // Should see both items from both caches
        assert_eq!(cache1.size(), 2);
        assert_eq!(cache2.size(), 2);
    }
}

fn main() {}