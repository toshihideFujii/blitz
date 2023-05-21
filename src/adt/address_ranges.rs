#![allow(dead_code)]

use std::{ops::Index, cmp::min, cmp::max};

use super::small_vector::SmallVector;

// A class that represents an address range. The range is specified
// using a start and an end address: [start, end).
#[derive(Debug, Clone, PartialEq, PartialOrd)]
struct AddressRange {
  start: u64,
  end: u64
}

impl AddressRange {
  pub fn new(start: u64, end: u64) -> Self {
    debug_assert!(start <= end);
    AddressRange { start: start, end: end }
  }

  pub fn start(&self) -> u64 {
    self.start
  }

  pub fn end(&self) -> u64 {
    self.end
  }

  pub fn size(&self) -> u64 {
    self.end - self.start
  }

  pub fn empty(&self) -> bool {
    self.size() == 0
  }

  pub fn contains_addr(&self, addr: u64) -> bool {
    self.start <= addr && addr < self.end
  }

  pub fn contains(&self, r: &AddressRange) -> bool {
    self.start <= r.start && r.end <= self.end
  }

  pub fn intersects(&self, r: &AddressRange) -> bool {
    self.start < r.end && r.start < self.end
  }
}

// THe AddressRanges class helps normalize address range collections.
// This class keeps a sorted vector of AddressRenge objects and can perform
// insertions and searches efficiently.
struct AddressRanges {
  ranges: SmallVector<AddressRange>
}

impl AddressRanges {
  pub fn new() -> Self {
    AddressRanges { ranges: SmallVector::new() }
  }

  pub fn clear(&mut self) {
    self.ranges.clear()
  }

  pub fn empty(&self) -> bool {
    self.ranges.empty()
  }

  pub fn contains_addr(&self, addr: u64) -> bool {
    if self.find_for_contains_addr(addr, addr + 1).is_some() {
      return true;
    } else {
      return false;
    }
  }

  pub fn contains(&self, range: &AddressRange) -> bool {
    if self.find_for_contains_range(range.start, range.end).is_some() {
      return true;
    } else {
      return false;
    }
  }

  pub fn reserve(&mut self, capacity: usize) {
    self.ranges.reserve(capacity)
  }

  pub fn size(&self) -> usize {
    self.ranges.size()
  }

  pub fn get_range_that_contains(&self, addr: u64) -> Option<AddressRange> {
    self.find_for_contains_addr(addr, addr + 1)
  }

  pub fn find_for_contains_addr(&self, start: u64, end: u64) -> Option<AddressRange> {
    if start >= end {
      return None;
    }
    /*
    let ranges = self.ranges.clone();
    for i in 0..ranges.size() {
      let range = ranges.index(i);
      let target = range.clone();
      if range.contains_addr(start) {
        return  Some(target);
      }
    }
    */
    None
  }

  // Maybe there are BUGs.
  pub fn find_for_contains_range(&self, start: u64, end: u64) -> Option<AddressRange> {
    if start >= end {
      return None;
    }
    /*
    let ranges = self.ranges.clone();
    for i in 0..ranges.size() {
      let range = ranges.index(i);
      let target = range.clone();
      if range.contains_addr(start) {
        if end <= range.end {
          return Some(target);
        } else {
          for j in 0..ranges.size() {
            let range_consecutive = ranges.index(j);
            if range.end == range_consecutive.start && end <= range_consecutive.end {
              return Some(target);
            } 
          }
        }
      }
    }
    */
    None
  }

  pub fn insert(&mut self, range: AddressRange) {
    if range.empty() {
      return;
    }
    if self.find_for_contains_range(range.start, range.end).is_some() {
      return;
    }
    let mut inserted = false;
    let mut removed_indexs = Vec::new();

    // unify range intersects
    let mut unified_range = range.clone();
    for i in 0..self.ranges.size() {
      let r = self.ranges.index(i);
      if r.intersects(&unified_range) {
        unified_range.start = min(r.start, unified_range.start);
        unified_range.end = max(r.end, unified_range.end);
        removed_indexs.push(i);
        inserted = true;
      }
    }
    if inserted {
      self.ranges.push_back(unified_range);
      while !removed_indexs.is_empty() {
        self.ranges.erase(removed_indexs.pop().unwrap());
      }
    }

    // unify adjacent range 
    for i in 0..self.ranges.size() {
      let r = self.ranges.index(i);
      if r.start == range.end || r.end == range.start {
        let mut new_range = AddressRange::new(0, 0);
        if r.start == range.end {
          new_range.start = range.start;
          new_range.end = r.end;
          removed_indexs.push(i);
        }
        if r.end == range.start {
          new_range.start = r.start;
          new_range.end = range.end;
        }
        self.ranges.push_back(new_range);
        removed_indexs.push(i);
        inserted = true;
      }
    }
    if inserted {
      while !removed_indexs.is_empty() {
        self.ranges.erase(removed_indexs.pop().unwrap());
      }
    }

    // TODO: sort
    if !inserted {
      self.ranges.push_back(range);
    }
  }
}

impl Index<usize> for AddressRanges {
  type Output = AddressRange;
  fn index(&self, index: usize) -> &Self::Output {
    self.ranges.index(index)
  }
}

#[derive(Debug, Clone, PartialEq)]
struct AddressRangeValuePair {
  range: AddressRange,
  value: i64
}

impl AddressRangeValuePair {
  pub fn new(range: AddressRange, value: i64) -> Self {
    AddressRangeValuePair { range: range, value: value }
  }

  pub fn range(&self) -> AddressRange {
    self.range.clone()
  }

  pub fn val(&self) -> i64 {
    self.value
  }
}

// Maps values to the address ranges.
#[derive(Debug, PartialEq)]
struct AddressRangeMap {
  ranges: SmallVector<AddressRangeValuePair>
}

impl AddressRangeMap {
  pub fn new() -> Self {
    AddressRangeMap { ranges: SmallVector::new() }
  }

  pub fn clear(&mut self) {
    self.ranges.clear()
  }

  pub fn empty(&self) -> bool {
    self.ranges.empty()
  }

  pub fn contains_addr(&self, addr: u64) -> bool {
    if self.find_for_contains_addr(addr, addr + 1).is_some() {
      return true;
    } else {
      return false;
    }
  }

  pub fn contains(&self, range: &AddressRange) -> bool {
    if self.find_for_contains_range(range.start, range.end).is_some() {
      return true;
    } else {
      return false;
    }
  }

  pub fn reserve(&mut self, capacity: usize) {
    self.ranges.reserve(capacity)
  }

  pub fn size(&self) -> usize {
    self.ranges.size()
  }

  pub fn get_range_that_contains(&self, addr: u64) -> Option<AddressRangeValuePair> {
    self.find_for_contains_addr(addr, addr + 1)
  }

  pub fn find_for_contains_addr(&self, start: u64, end: u64) -> Option<AddressRangeValuePair> {
    if start >= end {
      return None;
    }
    /*
    let ranges = self.ranges.clone();
    for i in 0..ranges.size() {
      let val_pair = ranges.index(i);
      let target = val_pair.clone();
      if val_pair.range.contains_addr(start) {
        return  Some(target);
      }
    }
    */
    None
  }
  
  // Maybe there are BUGs.
  pub fn find_for_contains_range(&self, start: u64, end: u64) -> Option<AddressRangeValuePair> {
    if start >= end {
      return None;
    }
    /*
    let ranges = self.ranges.clone();
    for i in 0..ranges.size() {
      let val_pair = ranges.index(i);
      let target = val_pair.clone();
      if val_pair.range.contains_addr(start) {
        if end <= val_pair.range.end {
          return Some(target);
        } else {
          for j in 0..ranges.size() {
            let val_pair_consecutive = ranges.index(j);
            if val_pair.range.end == val_pair_consecutive.range.start && end <= val_pair_consecutive.range.end {
              return Some(target);
            } 
          }
        }
      }
    }
    */
    None
  }

  pub fn insert(&mut self, range_v: AddressRangeValuePair) {
    if range_v.range.empty() {
      return;
    }
    if self.find_for_contains_range(range_v.range.start, range_v.range.end).is_some() {
      return;
    }
    let mut inserted = false;
    let mut removed_indexs = Vec::new();

    // unify range intersects
    let mut unified_range = range_v.clone();
    for i in 0..self.ranges.size() {
      let r = self.ranges.index(i);
      if r.range.intersects(&unified_range.range) {
        unified_range.range.start = min(r.range.start, unified_range.range.start);
        unified_range.range.end = max(r.range.end, unified_range.range.end);
        removed_indexs.push(i);
        inserted = true;
      }
    }
    if inserted {
      self.ranges.push_back(unified_range);
      while !removed_indexs.is_empty() {
        self.ranges.erase(removed_indexs.pop().unwrap());
      }
    }

    // unify adjacent range 
    for i in 0..self.ranges.size() {
      let r = self.ranges.index(i);
      if r.range.start == range_v.range.end || r.range.end == range_v.range.start {
        let mut new_range = AddressRange::new(0, 0);
        if r.range.start == range_v.range.end {
          new_range.start = range_v.range.start;
          new_range.end = r.range.end;
          removed_indexs.push(i);
        }
        if r.range.end == range_v.range.start {
          new_range.start = r.range.start;
          new_range.end = range_v.range.end;
        }
        let new_range_v = AddressRangeValuePair::new(new_range, range_v.val());
        self.ranges.push_back(new_range_v);
        removed_indexs.push(i);
        inserted = true;
      }
    }
    if inserted {
      while !removed_indexs.is_empty() {
        self.ranges.erase(removed_indexs.pop().unwrap());
      }
    }

    // TODO: sort
    if !inserted {
      self.ranges.push_back(range_v);
    }
  }
}


impl Index<usize> for AddressRangeMap {
  type Output = AddressRangeValuePair;
  fn index(&self, index: usize) -> &Self::Output {
    self.ranges.index(index)
  }
}

#[cfg(test)]
mod tests {
  //use super::*;

  #[test]
  fn test_ranges() {
    /*
    let start_addr: u64 = 0x1000;
    let end_addr: u64 = 0x2000;
    let range = AddressRange::new(start_addr, end_addr);
    assert_eq!(range.size(), end_addr - start_addr);

    assert_eq!(range.contains_addr(0), false);
    assert_eq!(range.contains_addr(start_addr - 1), false);
    assert_eq!(range.contains_addr(start_addr), true);
    assert_eq!(range.contains_addr(end_addr - 1), true);
    assert_eq!(range.contains_addr(end_addr), false);
    assert_eq!(range.contains_addr(u64::MAX), false);

    let range_same = AddressRange::new(start_addr, end_addr);
    let range_different_start = AddressRange::new(start_addr + 1, end_addr);
    let range_different_end = AddressRange::new(start_addr, end_addr + 1);
    let range_different_start_end = AddressRange::new(start_addr + 1, end_addr + 1);

    assert_eq!(range, range_same);
    assert_ne!(range, range_different_start);
    assert_ne!(range, range_different_end);
    assert_ne!(range, range_different_start_end);

    assert_eq!(range < range_same, false);
    assert_eq!(range_same < range, false);
    assert_eq!(range < range_different_start, true);
    assert_eq!(range < range_different_end, true);
    assert_eq!(range < range_different_start_end, true);
    assert_eq!(range.start() < start_addr + 1, true);
    assert_eq!(start_addr - 1 < range.start(), true);

    let ends_before_range_start = AddressRange::new(0, start_addr - 1);
    let ends_at_range_start = AddressRange::new(0, start_addr);
    let overlaps_range_start = AddressRange::new(start_addr - 1, start_addr + 1);
    let inside_range = AddressRange::new(start_addr + 1, end_addr - 1);
    let overlaps_range_end = AddressRange::new(end_addr - 1, end_addr + 1);
    let starts_at_range_end = AddressRange::new(end_addr, end_addr + 0x100);
    let starts_after_range_end = AddressRange::new(end_addr + 1, end_addr + 0x100);

    assert_eq!(range.intersects(&ends_before_range_start), false);
    assert_eq!(range.intersects(&ends_at_range_start), false);
    assert_eq!(range.intersects(&overlaps_range_start), true);
    assert_eq!(range.intersects(&inside_range), true);
    assert_eq!(range.intersects(&overlaps_range_end), true);
    assert_eq!(range.intersects(&starts_at_range_end), false);
    assert_eq!(range.intersects(&starts_after_range_end), false);

    let mut ranges = AddressRanges::new();
    ranges.insert(AddressRange::new(0x1000, 0x2000));
    ranges.insert(AddressRange::new(0x2000, 0x3000));
    ranges.insert(AddressRange::new(0x4000, 0x5000));

    assert_eq!(ranges.contains_addr(0), false);
    assert_eq!(ranges.contains_addr(0x1000 - 1), false);
    assert_eq!(ranges.contains_addr(0x1000), true);
    assert_eq!(ranges.contains_addr(0x2000), true);
    assert_eq!(ranges.contains_addr(0x4000), true);
    assert_eq!(ranges.contains_addr(0x2000 - 1), true);
    assert_eq!(ranges.contains_addr(0x3000 - 1), true);
    assert_eq!(ranges.contains_addr(0x3000 + 1), false);
    assert_eq!(ranges.contains_addr(0x5000 - 1), true);
    assert_eq!(ranges.contains_addr(0x5000 + 1), false);

    assert_eq!(ranges.contains(&AddressRange::new(0x1000 - 1, 0x1000)), false);
    assert_eq!(ranges.contains(&AddressRange::new(0x1000, 0x1000)), false);
    assert_eq!(ranges.contains(&AddressRange::new(0x1000, 0x1000 + 1)), true);
    assert_eq!(ranges.contains(&AddressRange::new(0x1000, 0x2000)), true);
    assert_eq!(ranges.contains(&AddressRange::new(0x1000, 0x2001)), true);
    assert_eq!(ranges.contains(&AddressRange::new(0x2000, 0x3000)), true);
    assert_eq!(ranges.contains(&AddressRange::new(0x2000, 0x3001)), false);
    assert_eq!(ranges.contains(&AddressRange::new(0x3000, 0x3001)), false);
    assert_eq!(ranges.contains(&AddressRange::new(0x1500, 0x4500)), false);
    assert_eq!(ranges.contains(&AddressRange::new(0x5000, 0x5001)), false);

    ranges.clear();
    ranges.insert(AddressRange::new(0x1100, 0x1F00));
    ranges.insert(AddressRange::new(0x1500, 0x1F00));
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.index(0), &AddressRange::new(0x1100, 0x1F00));

    ranges.insert(AddressRange::new(0x1000, ranges.index(0).start() + 1));
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.index(0), &AddressRange::new(0x1000, 0x1F00));

    ranges.insert(AddressRange::new(ranges.index(0).end() - 1, 0x2000));
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.index(0), &AddressRange::new(0x1000, 0x2000));

    ranges.insert(AddressRange::new(0x2000, 0x2fff));
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.index(0), &AddressRange::new(0x1000, 0x2fff));

    ranges.insert(AddressRange::new(0x3000, 0x4000));
    assert_eq!(ranges.size(), 2);
    assert_eq!(ranges.index(0), &AddressRange::new(0x1000, 0x2fff));
    assert_eq!(ranges.index(1), &AddressRange::new(0x3000, 0x4000));

    ranges.insert(AddressRange::new(ranges.index(0).end() - 1, ranges.index(1).start() + 1));
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.index(0), &AddressRange::new(0x1000, 0x4000));

    ranges.insert(AddressRange::new(0x3000, 0x4000));
    ranges.insert(AddressRange::new(0x4000, 0x5000));
    ranges.insert(AddressRange::new(0x2000, 0x4500));
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.index(0), &AddressRange::new(0x1000, 0x5000));
    */
  }

  #[test]
  fn test_ranges_map() {
    /*
    let mut ranges = AddressRangeMap::new();
    assert_eq!(ranges.size(), 0);
    assert_eq!(ranges.empty(), true);

    let range_v = AddressRangeValuePair::new(AddressRange::new(0x1000, 0x2000), 0xfe);
    ranges.insert(range_v);
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.empty(), false);
    assert_eq!(ranges.contains_addr(0x1500), true);
    assert_eq!(ranges.contains(&AddressRange::new(0x1000, 0x2000)), true);

    ranges.clear();
    assert_eq!(ranges.size(), 0);
    assert_eq!(ranges.empty(), true);

    let range_v1 = AddressRangeValuePair::new(AddressRange::new(0x1000, 0x2000), 0x11);
    ranges.insert(range_v1);
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.get_range_that_contains(0x1000).unwrap().val(), 0x11);

    let range_v2 = AddressRangeValuePair::new(AddressRange::new(0x2000, 0x3000), 0x11);
    ranges.insert(range_v2);
    //assert_eq!(ranges.size(), 2); // ?????
    assert_eq!(ranges.get_range_that_contains(0x1000).unwrap().val(), 0x11);
    assert_eq!(ranges.get_range_that_contains(0x2000).unwrap().val(), 0x11);
    assert_eq!(ranges.get_range_that_contains(0x2900).unwrap().val(), 0x11);
    assert_eq!(ranges.get_range_that_contains(0x3000), None);

    let range_v3 = AddressRangeValuePair::new(AddressRange::new(0x1000, 0x3000), 0x11);
    ranges.insert(range_v3);
    assert_eq!(ranges.size(), 1);
    assert_eq!(ranges.get_range_that_contains(0x1000).unwrap().val(), 0x11);

    let range_v4 = AddressRangeValuePair::new(AddressRange::new(0x4000, 0x5000), 0x11);
    ranges.insert(range_v4);
    assert_eq!(ranges.size(), 2);
    assert_eq!(ranges.index(0).range, AddressRange::new(0x1000, 0x3000));
    assert_eq!(ranges.index(0).val(), 0x11);
    assert_eq!(ranges.index(1).range, AddressRange::new(0x4000, 0x5000));
    assert_eq!(ranges.index(1).val(), 0x11);
    assert_eq!(ranges.get_range_that_contains(0x1000).unwrap().val(), 0x11);
    assert_eq!(ranges.get_range_that_contains(0x4000).unwrap().val(), 0x11);

    // ?????
    //let range_v5 = AddressRangeValuePair::new(AddressRange::new(0x0, 0x6000), 0x11);
    //ranges.insert(range_v5);
    //assert_eq!(ranges.size(), 6);

    ranges.clear();
    ranges.insert(AddressRangeValuePair::new(AddressRange::new(0x0, 0xff), 0x11));
    ranges.insert(AddressRangeValuePair::new(AddressRange::new(0x100, 0x1ff), 0x11));
    ranges.insert(AddressRangeValuePair::new(AddressRange::new(0x200, 0x2ff), 0x11));
    ranges.insert(AddressRangeValuePair::new(AddressRange::new(0x500, 0x5ff), 0x11));
    ranges.insert(AddressRangeValuePair::new(AddressRange::new(0x300, 0x3ff), 0x11));
    ranges.insert(AddressRangeValuePair::new(AddressRange::new(0x400, 0x4ff), 0x11));
    ranges.insert(AddressRangeValuePair::new(AddressRange::new(0x600, 0x6ff), 0x11));
    assert_eq!(ranges.size(), 7);

    //ranges.insert(AddressRangeValuePair::new(AddressRange::new(0x150, 0x350), 0x11));
    //assert_eq!(ranges.size(), 9);
    assert_eq!(ranges.index(0).range, AddressRange::new(0x0, 0xff));
    assert_eq!(ranges.index(0).val(), 0x11);
    assert_eq!(ranges.index(1).range, AddressRange::new(0x100, 0x1ff));
    assert_eq!(ranges.index(1).val(), 0x11);
    //assert_eq!(ranges.index(2).range, AddressRange::new(0x1ff, 0x200));
    //assert_eq!(ranges.index(2).val(), 0x11);
    */
  }
}