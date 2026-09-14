use std::collections::HashSet;

use rand::RngExt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ResourceKind {
  // Side-effecting operations (i.e., infeed and outfeed) define their
  // execution order via token dependencies. We rely on token resource to
  // enforce ordering at run time.
  Token,
  // Collective operations must be scheduled in the same order as they are
  // defined in the HLO module. We rely on collective communicator resource
  // to enforce ordering at run time.
  CollectiveComunicator,
}

// `Resource` models a run time resource that imposes ordering on the thunk
// execution (scheduling) in addition to thunk buffer uses.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Resource {
  kind: ResourceKind,
  rng: u64 // For distinguish cloned objects.
}

impl Resource {
  pub fn new(kind: ResourceKind) -> Self {
    let rng: u64 = rand::rng().random();
    Resource { kind, rng }
  }

  pub fn kind(&self) -> ResourceKind {
    self.kind.clone()
  }
}

// ResourceReadWriteSet tracks a set of read and write resources.
#[derive(Debug, Clone)]
pub struct ResourceReadWriteSet {
  read: HashSet<Resource>,
  write: HashSet<Resource>
}

impl ResourceReadWriteSet {
  pub fn default() -> Self {
    ResourceReadWriteSet { read: HashSet::new(), write: HashSet::new() }
  }

  pub fn add(&mut self, r_use: ResourceUse) {
    match r_use.access() {
      ResourceAccess::Read => self.read.insert(r_use.resource()),
      ResourceAccess::Write => self.write.insert(r_use.resource())
    };
  }

  pub fn add_all(&mut self, r_uses: Vec<ResourceUse>) {
    for r_use in r_uses {
      self.add(r_use);
    }
  }

  // Returns true if any of the resource use(s) has a conflict with tracked
  // resource reads or writes.
  pub fn has_conflicts_by_use(&self, r_use: &ResourceUse) -> bool {
    if r_use.access()== ResourceAccess::Write {
      return self.write.contains(&r_use.resource()) || self.read.contains(&r_use.resource());
    } else {
      return self.write.contains(&r_use.resource());
    }
  }

  pub fn has_conflicts_by_uses(&self, r_uses: &Vec<ResourceUse>) -> bool {
    for r_use in r_uses {
      if self.has_conflicts_by_use(r_use) { return true; }
    }
    false
  }

  pub fn has_conflicts_by_other(&self, other: &ResourceReadWriteSet) -> bool {
    let mut other_read = false;
    for read in &other.read {
      if self.has_conflicts_by_use(&ResourceUse::read(read.clone())) {
        other_read = true;
      }
    }
    let mut other_write = false;
    for write in &other.write {
      if self.has_conflicts_by_use(&ResourceUse::write(write.clone())) {
        other_write = true;
      }
    }
    other_read || other_write
  }

  // Collects all resource uses that have a conflict with tracked resource
  // reads or writes.
  pub fn conflicts(&self, other: &ResourceReadWriteSet) -> Vec<ResourceUse> {
    let mut conflicts = vec![];
    for resource in &other.read {
      let read = ResourceUse::read(resource.clone());
      if self.has_conflicts_by_use(&read) {
        conflicts.push(read);
      }
    }
    for resource in &other.write {
      let write = ResourceUse::write(resource.clone());
      if self.has_conflicts_by_use(&write) {
        conflicts.push(write);
      }
    }
    conflicts
  }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ResourceAccess {
  Read,
  Write
}

// For consistency with BufferUse, we model resource uses as writes or reads
// to and from resource. Resources have referential equality: we rely on
// comparing pointers to check if resource is the same or not.
//
// Examples of using resources in BLITZ:
//
//   - HLO control dependencies that are not representable as buffers also
//     modeled as resource writes and reads.
//   - in BLITZ:CPU all collective operations must be ordered at run time. We use
//     a global "collective communication" resource to model this constraint.
//
#[derive(Debug, Clone, PartialEq)]
pub struct ResourceUse {
  resource: Resource,
  access: ResourceAccess
}

impl ResourceUse {
  pub fn new(resource: Resource, access: ResourceAccess) -> Self {
    ResourceUse { resource, access }
  }

  pub fn resource(&self) -> Resource {
    self.resource.clone()
  }

  pub fn read(resource: Resource) -> Self {
    ResourceUse::new(resource, ResourceAccess::Read)
  }

  pub fn write(resource: Resource) -> Self {
    ResourceUse::new(resource, ResourceAccess::Write)
  }

  pub fn access(&self) -> ResourceAccess {
    self.access.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_equality() {
    let token = Resource::new(ResourceKind::Token);
    let use_0 = ResourceUse::read(token.clone());
    let use_1 = ResourceUse::write(token.clone());
    let use_2 = ResourceUse::read(token.clone());

    assert_ne!(use_0, use_1);
    assert_eq!(use_0, use_2);    
  }

  #[test]
  fn test_read_write_set() {
    let mut rwset = ResourceReadWriteSet::default();
    let token_0 = Resource::new(ResourceKind::Token);
    let token_1 = Resource::new(ResourceKind::Token);

    rwset.add(ResourceUse::read(token_0.clone()));
    assert!(!rwset.has_conflicts_by_use(&ResourceUse::read(token_0.clone())));
    assert!(rwset.has_conflicts_by_use(&ResourceUse::write(token_0.clone())));
    assert!(!rwset.has_conflicts_by_use(&ResourceUse::read(token_1.clone())));
    assert!(!rwset.has_conflicts_by_use(&ResourceUse::write(token_1.clone())));

    rwset.add(ResourceUse::write(token_0.clone()));
    assert!(rwset.has_conflicts_by_use(&ResourceUse::read(token_0.clone())));
    assert!(rwset.has_conflicts_by_use(&ResourceUse::write(token_0.clone())));
    assert!(!rwset.has_conflicts_by_use(&ResourceUse::read(token_1.clone())));
    assert!(!rwset.has_conflicts_by_use(&ResourceUse::write(token_1.clone())));

    let mut rwset_2 = ResourceReadWriteSet::default();
    rwset_2.add(ResourceUse::write(token_0.clone()));
    let conflicts = rwset.conflicts(&rwset_2);
    assert_eq!(conflicts.len(), 1);
    assert_eq!(conflicts.first().unwrap(), &ResourceUse::write(token_0.clone()));
  }
}