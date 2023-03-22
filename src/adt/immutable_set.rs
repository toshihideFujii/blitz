#![allow(dead_code)]

#[derive(Debug, Clone)]
struct ImutAVLTree<V> {
  left: Option<Box<ImutAVLTree<V>>>,
  right: Option<Box<ImutAVLTree<V>>>,
  prev: Option<Box<ImutAVLTree<V>>>,
  next: Option<Box<ImutAVLTree<V>>>,
  height: u32,
  is_mutable: bool,
  is_digest_cached: bool,
  is_canonicalized: bool,
  value: V,
  digest: u32,
  ref_count: u32
}

impl<V> ImutAVLTree<V> where V: Clone + PartialOrd {
  pub fn new() {}

  // Return a pointer to the left subtree.
  pub fn get_left(&self) -> Option<Box<ImutAVLTree<V>>> {
    self.left.clone()
  }

  // Return a pointer to the right subtree.
  pub fn get_right(&self) -> Option<Box<ImutAVLTree<V>>> {
    self.right.clone()
  }

  // Returns the height of the tree.
  pub fn get_height(&self) -> u32 {
    self.height
  }

  // Returns the data value associated with the tree node.
  pub fn get_value(&self) -> V {
    self.value.clone()
  }

  // Finds the subtree associated with the specified key value.
  pub fn find(&self, key: V) -> Option<Box<ImutAVLTree<V>>> {
    let mut tree = Some(Box::new(self.clone()));
    while tree.is_some() {
      let tree_c = tree.clone();
      let current_key = ImutContainerInfo::key_of_value(&tree_c.unwrap().get_value());
      if ImutContainerInfo::is_equal(&key, &current_key) {
        return tree;
      } else if ImutContainerInfo::is_less(&key, &current_key) {
        tree = tree.unwrap().get_left();
      } else {
        tree = tree.unwrap().get_right();
      }
    }
    None
  }

  // Find the subtree associated with the highest ranged key value.
  pub fn get_max_element(&self) -> Option<Box<ImutAVLTree<V>>> {
    let mut right = self.get_right();
    while right.is_some() {
      right = right.unwrap().get_right();
    }
    right
  }

  // Returns the number of nodes in the tree, which includes both
  // leaves and non-leaf nodes.
  pub fn size(&self) -> u32 {
    let mut n = 1;
    if self.get_left().is_some() {
      n += self.get_left().unwrap().size();
    }
    if self.get_right().is_some() {
      n += self.get_right().unwrap().size();
    }
    n
  }

  pub fn is_element_equal(&self, v: V) -> bool {
    let lhs_k = ImutContainerInfo::key_of_value(&self.get_value());
    let rhs_k = ImutContainerInfo::key_of_value(&v);
    if !ImutContainerInfo::is_equal(&lhs_k, &rhs_k) {
      return false;
    }
    let lhs_v = ImutContainerInfo::data_of_value(&self.get_value());
    let rhs_v = ImutContainerInfo::data_of_value(&v);
    if !ImutContainerInfo::is_data_equal(&lhs_v, &rhs_v) {
      return false;
    }
    true
  }

  pub fn is_element_equal_from_tree(&self, rhs: ImutAVLTree<V>) -> bool {
    self.is_element_equal(rhs.get_value())
  }

  // Returns true if this tree contains a subtree (node) that has an
  // data element that matches the specified key.
  pub fn contains(&self, k: V) -> bool {
    if self.find(k).is_none() {
      return false;
    }
    true
  }

  pub fn validate_tree(&self) {}

  // Returns true if the left and right subtree references (as well as
  // height) can be changed.
  fn is_mutable(&self) -> bool {
    self.is_mutable
  }

  // Returns true if the digest for this tree is cached.
  fn has_cached_digest(&self) -> bool {
    self.is_digest_cached
  }

  // Clears the mutable flag for a tree.
  fn mark_immutable(&mut self) {
    debug_assert!(self.is_mutable, "Mutable flag already removed.");
    self.is_mutable = false
  }

  // Clears the NoCachedDigest flag for a tree.
  fn marked_cached_digest(&mut self) {
    debug_assert!(!self.has_cached_digest(), "NoCachedDigest flag already removed.");
    self.is_digest_cached = true
  }

  // Changes the height of the tree.
  fn set_height(&mut self, h: u32) {
    debug_assert!(self.is_mutable(), "Only a mutable tree can have its height changed.");
    self.height = h;
  }

  fn compute_digest_from(&self, _l: Option<Box<ImutAVLTree<V>>>,
    _r: Option<Box<ImutAVLTree<V>>>, _v: V) -> u32 {
    0
  }

  fn compute_digest(&mut self) -> u32 {
    if self.has_cached_digest() {
      return self.digest;
    }
    let x = self.compute_digest_from(self.get_left(),
      self.get_right(), self.get_value());
    self.digest = x;
    self.marked_cached_digest();
    x
  }

  pub fn retain() {}
  pub fn release() {}
  pub fn destroy() {}
}

struct ImutAVLFactory {}
impl ImutAVLFactory {
  pub fn add() {}
  pub fn remove() {}
  pub fn get_empty_tree() {}
  pub fn is_empty() {}
  pub fn get_height() {}
  pub fn get_left() {}
  pub fn get_right() {}
  pub fn get_value() {}
  pub fn mask_cache_index() {}
  pub fn increment_height() {}
  pub fn compare_tree_with_section() {}
  pub fn create_node() {}
  pub fn recover_nodes() {}
  pub fn balance_tree() {}
  pub fn add_internal() {}
  pub fn remove_internal() {}
  pub fn combine_trees() {}
  pub fn remove_min_binding() {}
  pub fn mark_immutable() {}
  pub fn get_canonical_tree() {}
}

struct ImutProfileInfo<T> {
  type_: T
}
impl<T> ImutProfileInfo<T> {
  pub fn profile() {}
}

// Trait classes that contain element comparison operators and type
// definitions used by ImutAVLTree, ImmutableSet, and ImmutableMap.
// These inherit from the profile traits (ImutProfileInfo) to include
// operations for element profiling.
struct ImutContainerInfo<T> {
  value_type: T
}
impl<T> ImutContainerInfo<T> where T: Clone + PartialEq + PartialOrd {
  pub fn key_of_value(value: &T) -> T {
    value.clone()
  }

  pub fn data_of_value(_value: &T) -> bool {
    true
  }

  pub fn is_equal(lhs: &T, rhs: &T) -> bool {
    lhs == rhs
  }

  pub fn is_less(lhs: &T, rhs: &T) -> bool {
    lhs < rhs

  }
  pub fn is_data_equal(_lhs: &T, _rhs: &T) -> bool {
    true
  }
}

struct ImmutableSet<T> {
  root: ImutAVLTree<T>
}
impl<T> ImmutableSet<T> where T: Clone + PartialOrd {
  pub fn new() {}

  // Return true if the set contains the specified value.
  pub fn contains(&self, v: T) -> bool {
    self.root.contains(v)
  }

  pub fn get_root(&self) -> ImutAVLTree<T> {
    self.root.clone()
  }

  pub fn get_root_without_retain() {}

  // Return true if the set contains exactly one element.
  pub fn is_singleton(&self) -> bool {
    self.get_height() == 1
  }

  pub fn get_height(&self)-> u32 {
    self.root.get_height()
  }

  pub fn profile() {}

  pub fn validate_tree(&self) {
    self.root.validate_tree()
  }
}