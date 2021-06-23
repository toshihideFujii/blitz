

// HeapObject is the superclass for all classes describing heap
// allocated objects.
struct HeapObject {

}

impl HeapObject {
    pub fn is_null(self) -> bool {
        false
    }

    // [map]: Contains a map which contains the object's reflective information.
    pub fn set_map(self) {}

    pub fn map_slot() {}

    pub fn set_map_no_write_barrier() {}

    // Compare-and-swaps map word using release store, returns true if
    // the map was actually swapped.
    pub fn release_compare_and_swap_map_word() -> bool {
        false
    }

    // Initialize the map immediately after the object is allocated.
    // Do not use this outside Heap.
    pub fn set_map_after_allocation() {}

    // Access the map word using acquire load and release store.
    pub fn set_map_word() {}

    pub fn get_read_only_roots() {}

    pub fn is_external() {}

    pub fn from_address() {}
}