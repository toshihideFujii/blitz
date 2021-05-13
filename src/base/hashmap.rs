
#[allow(unused_imports)]
#[allow(unused_variables)]
//#[allow(dead_code)]

use super::hashmap_entry;

// The default capacity.
// This is used by call sites which want to pass in a non-default AllocationPolicy
// but want to use the default value of capacity specified by the implementation.
const DEFAULT_HASH_MAP_CAPACITY: u32 = 8;

//type Entry = TemplateHashMapEntry<Key, Value>;

struct TemplateHashMap<Key, Value, /*MatchFun, AllocationPolicy*/> {
    key: Key,
    val: Value
}

impl<Key, Value/*, MatchFun, AllocationPolicy*/> TemplateHashMap<Key, Value/*, MatchFun, AllocationPolicy*/> {

    // If an entry with matching key is found, returns that entry.
    // Otherwise, nullptr is returned.
    fn lookup(/*key: Key, hash: u32*/) {
    }

    // If an entry with matching key is found, returns that entry.
    // If no matching entry is found, a new entry is inserted with corresponding key,
    // key hash, and default initialized value.
    fn lookup_or_insert(/*key: Key, hash: u32*/) {
    }

    fn insert_new(/*key: Key, hash: u32*/) {
    }

    // Removes the entry with matching key.
    // It returns the value of the deleted entry or null if there is no value for such key.
    fn remove(self, /*key: Key, hash: u32*/) -> Value {
        self.val
    }

    // Empties the hash map (occupancy() === 0).
    fn clear() {
    }

    // The number of (non-empty) entries in the table.
    fn occupancy() -> u32 {
        return 0;
    }

    fn start() {
    }

    fn next() {
    }

    fn allocator() {
    }

    fn resize() {}
}