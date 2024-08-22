#![allow(dead_code)]

// The TransferManager interface lets backends provide platform-specific
// mechanisms for constructing literals from given device memory handles.
// This lets each platform customize how literals are transferred to/from the
// device in terms of padding, leading dimension, etc.
pub struct TransferManager {}

impl TransferManager {
  pub fn new() {}
}