
use common::{array::Array, blitz_data::ReplicaGroup};
use crate::tile_assignment::IotaTileAssignment;

// Represents a list of replica groups (a list of list of devices) with
// reshaping and transposing an iota array (iota tile assignment). Can be used
// to represent certain common patterns of device lists in a compact, scalable
// format.
#[derive(Debug, Clone, PartialEq)]
pub struct IotaReplicaGroupList {
  iota_tile_assignment: IotaTileAssignment,
  num_replica_groups: i64,
  num_devices_per_group: i64,
}

impl IotaReplicaGroupList {
  pub fn new(num_replica_groups: i64, num_devices_per_group: i64) -> Self {
    IotaReplicaGroupList {
      iota_tile_assignment: IotaTileAssignment::create(
        &vec![num_replica_groups, num_devices_per_group]),
      num_replica_groups: num_replica_groups,
      num_devices_per_group: num_devices_per_group
    }
  }

  pub fn num_replica_groups(&self) -> i64 {
    self.num_replica_groups
  }

  pub fn num_devices_per_group(&self) -> i64 {
    self.num_devices_per_group
  }

  pub fn reshape_dims(&self) -> &Vec<i64> {
    self.iota_tile_assignment.reshape_dims()
  }

  pub fn transpose_perm(&self) -> &Vec<i64> {
    self.iota_tile_assignment.transpose_perm()
  }

  pub fn to_array(&self) -> Array {
    self.iota_tile_assignment.to_array()
  }

  pub fn to_string(&self) -> String {
    self.iota_tile_assignment.to_string()
  }
}

// Represents a series of devices participating in a collective operation
// (all-gather, all-reduce, etc.). While this directly translates to a list of
// replica groups, it may be used to represent these lists in compact forms.
#[derive(Debug, Clone)]
pub struct CollectiveDeviceList {
  iota_replica_group_list: Option<IotaReplicaGroupList>,
  replica_groups: Option<Vec<ReplicaGroup>>
}

impl CollectiveDeviceList {
  pub fn default() -> Self {
    CollectiveDeviceList {
      iota_replica_group_list: None,
      replica_groups: Some(Vec::new())
    }
  }

  pub fn new(replica_groups: Vec<ReplicaGroup>) -> Self {
    CollectiveDeviceList {
      iota_replica_group_list: None,
      replica_groups: Some(replica_groups)
    }
  }

  pub fn new_from_vec(replica_groups: &Vec<Vec<i64>>) -> Self {
    CollectiveDeviceList {
      iota_replica_group_list: None,
      replica_groups: Some(CollectiveDeviceList::to_replica_group_vector(replica_groups))
    }
  }

  // Replica groups are materialized lazily upon first access.
  pub fn new_from_iota_rg_list(iota_replica_group_list: IotaReplicaGroupList) -> Self {
    CollectiveDeviceList {
      iota_replica_group_list: Some(iota_replica_group_list),
      replica_groups: None
    }
  }

  pub fn replica_groups(&self) -> &Option<Vec<ReplicaGroup>> {
    &self.replica_groups
  }

  pub fn iota_replica_group_list(&self) -> &Option<IotaReplicaGroupList> {
    &self.iota_replica_group_list
  }

  pub fn to_string(&self, print_full_replica_group_list: bool) -> String {
    if self.iota_replica_group_list.is_some() && !print_full_replica_group_list {
      return self.iota_replica_group_list.as_ref().unwrap().to_string();
    }
    replica_groups_to_string(self.replica_groups().as_ref().unwrap())
  }

  fn to_replica_group_vector(replica_groups: &Vec<Vec<i64>>) -> Vec<ReplicaGroup> {
    let mut result: Vec<ReplicaGroup> = Vec::new();
    for g in replica_groups {
      let mut group = ReplicaGroup::new();
      group.mutable_replica_ids().clone_from(g);
      result.push(group);
    }
    result
  }

  #[allow(dead_code)]
  fn set_expand_iota_to_replica_group(&mut self) {
    assert!(self.iota_replica_group_list.is_some());
    unimplemented!()
  }

  #[allow(dead_code)]
  fn expand_iota(iota: &IotaReplicaGroupList) {
    let array = iota.to_array();
    // Iota replica group list array must only have 2 dimensions.
    assert_eq!(array.num_dimensions(), 2);
    unimplemented!()
  }
}

fn replica_groups_to_string(replica_groups: &Vec<ReplicaGroup>) -> String {
  let mut replica_g_str = vec![];
  for group in replica_groups {
    let mut str = "{".to_string();
    let mut counter = 0;
    for id in group.replica_ids() {
      counter += 1;
      str.push_str(&id.to_string());
      if counter == group.replica_ids().len() { break; }
      str.push_str(",");
    }
    str.push_str("}");
    replica_g_str.push(str);
  }

  let mut result = "{".to_string();
  let mut counter = 0;
  for g_str in &replica_g_str {
    counter += 1;
    result.push_str(g_str);
    if counter == replica_g_str.len() {  break; }
    result.push_str(",");
  }
  result.push_str("}");
  result
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_list_to_string() {
    assert_eq!(CollectiveDeviceList::default().to_string(true), "{}".to_string());
    assert_eq!(CollectiveDeviceList::default().to_string(false), "{}".to_string());

    let empty_group_1 = ReplicaGroup::new();
    let empty_group_2 = ReplicaGroup::new();
    let mut empty_groups = Vec::new();
    empty_groups.push(empty_group_1);
    empty_groups.push(empty_group_2);
    assert_eq!(CollectiveDeviceList::new(empty_groups).to_string(false), "{{},{}}");

    let empty_groups_2: Vec<Vec<i64>> = vec![vec![]];
    assert_eq!(CollectiveDeviceList::new_from_vec(&empty_groups_2).to_string(false),
      "{{}}".to_string());
    assert_eq!(CollectiveDeviceList::new_from_vec(&vec![vec![1]]).to_string(false),
      "{{1}}".to_string());
    assert_eq!(CollectiveDeviceList::new_from_vec(&vec![vec![1, 2], vec![3, 4]]).to_string(false),
      "{{1,2},{3,4}}".to_string());
    assert_eq!(CollectiveDeviceList::new_from_vec(&vec![vec![1, 2, 3, 4, 5, 6, 7]]).to_string(false),
      "{{1,2,3,4,5,6,7}}".to_string());
  }

  #[test]
  fn test_deep_copy() {
    let orig = CollectiveDeviceList::new_from_vec(
      &vec![vec![1, 2, 3, 4]]);
    let copy = orig.clone();
    assert_eq!(orig.replica_groups(), copy.replica_groups());
    assert_eq!(orig.to_string(false), copy.to_string(false));
  }

  #[test]
  fn test_deep_copy_iota_before_expansion() {
    let iota_rg_list =
      IotaReplicaGroupList::new(2, 4);
    let orig =
      CollectiveDeviceList::new_from_iota_rg_list(iota_rg_list);

    let copy = orig.clone();
    assert_eq!(orig.iota_replica_group_list(), copy.iota_replica_group_list());
    assert_eq!(orig.replica_groups(), copy.replica_groups());
    assert_eq!(orig.to_string(false), copy.to_string(false));
  }

  #[test]
  fn test_deep_copy_iota_after_expansion() {
    let iota_rg_list =
      IotaReplicaGroupList::new(2, 4);
    let orig =
      CollectiveDeviceList::new_from_iota_rg_list(iota_rg_list);

    let local_ref = &orig.replica_groups;
    let copy = orig.clone();
    assert_eq!(orig.iota_replica_group_list(), copy.iota_replica_group_list());
    assert_eq!(orig.replica_groups(), copy.replica_groups());
    assert_eq!(local_ref, copy.replica_groups());
    assert_eq!(orig.to_string(false), copy.to_string(false));
  }

  #[test]
  fn test_iota_to_string() {
    let list = CollectiveDeviceList::new_from_iota_rg_list(
      IotaReplicaGroupList::new(0, 0));
    assert_eq!(list.to_string(false), "[0,0]<=[0]".to_string());

    let list2 = CollectiveDeviceList::new_from_iota_rg_list(
      IotaReplicaGroupList::new(2, 10));
    assert_eq!(list2.to_string(false), "[2,10]<=[20]".to_string());
  }

  #[test] // Fail
  fn test_iota_to_replica_group_string() {
    let list = CollectiveDeviceList::new_from_iota_rg_list(
      IotaReplicaGroupList::new(2, 10));
    assert_eq!(list.to_string(false), "[2,10]<=[20]".to_string());
    assert_eq!(list.to_string(true),
      "{{0,1,2,3,4,5,6,7,8,9},{10,11,12,13,14,15,16,17,18,19}}".to_string());
  }
}