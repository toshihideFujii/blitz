#![allow(dead_code)]

use common::{array::Array, blitz_data::ReplicaGroup};

use crate::tile_assignment::IotaTileAssignment;

// Represents a list of replica groups (a list of list of devices) with
// reshaping and transposing an iota array (iota tile assignment). Can be used
// to represent certain common patterns of device lists in a compact, scalable
// format.
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

  pub fn to_array(&self) -> &Array<i64> {
    self.iota_tile_assignment.to_array()
  }

  pub fn to_string(&self) -> String {
    self.iota_tile_assignment.to_string()
  }
}

// Represents a series of devices participating in a collective operation
// (all-gather, all-reduce, etc.). While this directly translates to a list of
// replica groups, it may be used to represent these lists in compact forms.
pub struct CollectiveDeviceList {
  iota_replica_group_list: Option<IotaReplicaGroupList>,
  replica_groups: Option<Vec<ReplicaGroup>>
}

impl CollectiveDeviceList {
  pub fn new(replica_groups: Vec<ReplicaGroup>) -> Self {
    CollectiveDeviceList {
      iota_replica_group_list: None,
      replica_groups: Some(replica_groups)
    }
  }

  pub fn replica_groups(&self) -> &Vec<ReplicaGroup> {
    self.replica_groups.as_ref().unwrap()
  }

  pub fn iota_replica_group_list(&self) -> &Option<IotaReplicaGroupList> {
    &self.iota_replica_group_list
  }

  pub fn to_string(&self, print_full_replica_group_list: bool) -> String {
    if self.iota_replica_group_list.is_some() && !print_full_replica_group_list {
      return self.iota_replica_group_list.as_ref().unwrap().to_string();
    }
    replica_groups_to_string(self.replica_groups())
  }
}

fn replica_groups_to_string(replica_groups: &Vec<ReplicaGroup>) -> String {
  let mut replica_g_str = vec![];
  for group in replica_groups {
    let mut str = "{".to_string();
    for id in group.replica_ids() {
      str.push_str(&id.to_string());
      str.push_str(","); // TODO: Remove last comma.
    }
    str.push_str("}");
    replica_g_str.push(str);
  }

  let mut result = "{".to_string();
  for g_str in &replica_g_str {
    result.push_str(g_str);
    result.push_str(","); // TODO: Remove last comma.
  }
  result
}